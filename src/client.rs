pub mod any_helper;
pub mod auth;
pub mod authz;
pub mod bank;
pub mod distribution;
pub mod evidence;
pub mod feegrant;
pub mod gov;
pub mod mint;
pub mod params;
pub mod slashing;
pub mod staking;
pub mod tx;
pub mod upgrade;
pub mod wasm;

use crate::client::any_helper::{any_to_cosmos, CosmosType};
use crate::client::tx::Response;
use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{MsgDelegate, MsgUndelegate};
use cosmos_sdk_proto::cosmos::tx::v1beta1::{BroadcastMode, GetTxResponse};
use cosmos_sdk_proto::ibc::applications::transfer::v1::MsgTransfer;
use cosmos_sdk_proto::ibc::core::client::v1::Height;
use cosmos_sdk_proto::traits::MessageExt;
use cosmrs::tendermint::chain;
use cosmrs::tx::{Fee, SignDoc, SignerInfo};
use std::ops::{DivAssign, MulAssign};
use std::rc::Rc;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use tendermint_rpc::{Client, HttpClient};

use crate::error::CosmosClient;
use crate::error::CosmosClient::{AccountDoesNotExistOnChain, NoSignerAttached};
use crate::signer::Signer;
use crate::tx::Cosmos;

pub struct Rpc {
    chain_id: String,
    signer: Option<Signer>,
    account_id: Option<u64>,
    sequence_id: Option<u64>,
    pub bank: bank::Module,
    pub auth: auth::Module,
    pub authz: authz::Module,
    pub distribution: distribution::Module,
    pub evidence: evidence::Module,
    pub feegrant: feegrant::Module,
    pub gov: gov::Module,
    pub mint: mint::Module,
    pub params: params::Module,
    pub slashing: slashing::Module,
    pub staking: staking::Module,
    pub tx: tx::Module,
    pub upgrade: upgrade::Module,
    pub wasm: wasm::Module,
}

impl Rpc {
    /// # Errors
    ///
    /// Will return `Err` if :
    /// - rpc server is down or invalid
    pub async fn new(url: &str) -> Result<Self, CosmosClient> {
        let rpc = Rc::new(HttpClient::new(url)?);

        Ok(Rpc {
            chain_id: rpc.status().await?.node_info.network.to_string(),
            signer: None,
            account_id: None,
            sequence_id: None,
            auth: auth::Module::new(rpc.clone()),
            authz: authz::Module::new(rpc.clone()),
            bank: bank::Module::new(rpc.clone()),
            distribution: distribution::Module::new(rpc.clone()),
            evidence: evidence::Module::new(rpc.clone()),
            feegrant: feegrant::Module::new(rpc.clone()),
            gov: gov::Module::new(rpc.clone()),
            mint: mint::Module::new(rpc.clone()),
            params: params::Module::new(rpc.clone()),
            slashing: slashing::Module::new(rpc.clone()),
            staking: staking::Module::new(rpc.clone()),
            tx: tx::Module::new(rpc.clone()),
            upgrade: upgrade::Module::new(rpc.clone()),
            wasm: wasm::Module::new(rpc),
        })
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - we cannot update `sequence_id` for `signer`
    pub async fn attach_signer(&mut self, signer: Signer) -> Result<(), CosmosClient> {
        self.signer = Some(signer);
        self.update_sequence_id().await?;
        Ok(())
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - there is no signer attached
    /// - cosmos `account` endpoint fails
    pub async fn update_sequence_id(&mut self) -> Result<(), CosmosClient> {
        let signer = self.signer()?;

        let account = self
            .auth
            .account(signer.public_address.to_string().as_str())
            .await?;
        if let Some(account) = account.account {
            if let Ok(CosmosType::BaseAccount(account)) = any_to_cosmos(&account) {
                self.sequence_id = Some(account.sequence);
                self.account_id = Some(account.account_number);
                return Ok(());
            }
        }

        Err(AccountDoesNotExistOnChain {
            address: signer.public_address.to_string(),
        })
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - there is no signer attached
    /// - cosmos `simulate` endpoint fails
    /// - the is a sign or encode error
    pub async fn sign(&mut self, tx: Cosmos) -> Result<Vec<u8>, CosmosClient> {
        let account_id = self.account_id.ok_or(AccountDoesNotExistOnChain {
            address: self.signer()?.public_address.to_string(),
        })?;
        let sequence_id = self.sequence_id.ok_or(AccountDoesNotExistOnChain {
            address: self.signer()?.public_address.to_string(),
        })?;
        self.sequence_id = Some(self.sequence_id.unwrap_or_default() + 1u64);

        let signer = self.signer()?;

        let tx_body = tx.finish();
        let auth_info = SignerInfo::single_direct(Some(signer.public_key), sequence_id).auth_info(
            Fee::from_amount_and_gas(
                cosmrs::Coin {
                    amount: signer.gas_price,
                    denom: signer.denom.parse()?,
                },
                100u64,
            ),
        );

        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &chain::Id::from_str(self.chain_id.as_str())?,
            account_id,
        )?;
        let tx_raw = sign_doc.sign(&signer.private_key)?;
        let tx = self.tx.simulate(tx_raw.to_bytes()?).await?;

        if tx.gas_info.is_none() {
            return Err(CosmosClient::CannotSimulateTxGasFee);
        }

        let mut gas_info = tx.gas_info.unwrap_or_default().gas_used;
        gas_info.mul_assign(100u64 + u64::from(signer.gas_adjustment_percent));
        gas_info.div_assign(100);

        let auth_info = SignerInfo::single_direct(Some(signer.public_key), sequence_id).auth_info(
            Fee::from_amount_and_gas(
                cosmrs::Coin {
                    amount: signer.gas_price,
                    denom: signer.denom.parse()?,
                },
                gas_info,
            ),
        );

        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &chain::Id::from_str(self.chain_id.as_str())?,
            account_id,
        )?;

        Ok(sign_doc.sign(&signer.private_key)?.to_bytes()?)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - cosmos `tx` broadcast endpoint fails
    pub async fn broadcast(
        &mut self,
        payload: Vec<u8>,
        mode: BroadcastMode,
    ) -> Result<Response, CosmosClient> {
        self.tx.broadcast(payload, mode).await
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - sign or broadcast fails
    pub async fn sign_and_broadcast(
        &mut self,
        tx: Cosmos,
        mode: BroadcastMode,
    ) -> Result<Response, CosmosClient> {
        let payload = self.sign(tx).await?;

        self.tx.broadcast(payload, mode).await
    }

    fn signer(&self) -> Result<&Signer, CosmosClient> {
        self.signer.as_ref().ok_or(NoSignerAttached)
    }

    async fn poll_for_tx(&self, tx: Response) -> Result<GetTxResponse, CosmosClient> {
        let hash = match tx {
            Response::Sync(tx) => tx.hash,
            Response::Async(tx) => tx.hash,
            Response::Commit(tx) => tx.hash,
        };

        for _ in 0..60 {
            let tx = self.tx.get_tx(hash.to_string().as_str()).await;

            if tx.is_ok() {
                return tx;
            }
            sleep(Duration::from_secs(3));
        }

        Err(CosmosClient::TXPollingTimeout)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - `sign_and_broadcast` returns an err
    /// - cannot find the hash of the tx on chain after 60'
    /// - cannot Serialize `MsgSend`
    pub async fn send(
        &mut self,
        to: &str,
        coin: Vec<Coin>,
        memo: Option<&str>,
    ) -> Result<GetTxResponse, CosmosClient> {
        let signer = self.signer()?;

        let mut payload = Cosmos::build().add_msg(
            MsgSend {
                from_address: signer.public_address.to_string(),
                to_address: to.to_string(),
                amount: coin,
            }
            .to_any()?,
        );
        if let Some(memo) = memo {
            payload = payload.memo(memo);
        }

        let tx = self
            .sign_and_broadcast(payload, BroadcastMode::Sync)
            .await?;
        self.poll_for_tx(tx).await
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - `sign_and_broadcast` returns an err
    /// - cannot find the hash of the tx on chain after 60'
    /// - cannot Serialize `MsgDelegate`
    pub async fn stake(
        &mut self,
        to: &str,
        coin: Coin,
        memo: Option<&str>,
    ) -> Result<GetTxResponse, CosmosClient> {
        let signer = self.signer()?;

        let mut payload = Cosmos::build().add_msg(
            MsgDelegate {
                delegator_address: signer.public_address.to_string(),
                validator_address: to.to_string(),
                amount: Some(coin),
            }
            .to_any()?,
        );
        if let Some(memo) = memo {
            payload = payload.memo(memo);
        }

        let tx = self
            .sign_and_broadcast(payload, BroadcastMode::Sync)
            .await?;
        self.poll_for_tx(tx).await
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - `sign_and_broadcast` returns an err
    /// - cannot find the hash of the tx on chain after 60'
    /// - cannot Serialize `MsgUndelegate`
    pub async fn unstake(
        &mut self,
        to: &str,
        coin: Coin,
        memo: Option<&str>,
    ) -> Result<GetTxResponse, CosmosClient> {
        let signer = self.signer()?;

        let mut payload = Cosmos::build().add_msg(
            MsgUndelegate {
                delegator_address: signer.public_address.to_string(),
                validator_address: to.to_string(),
                amount: Some(coin),
            }
            .to_any()?,
        );
        if let Some(memo) = memo {
            payload = payload.memo(memo);
        }

        let tx = self
            .sign_and_broadcast(payload, BroadcastMode::Sync)
            .await?;
        self.poll_for_tx(tx).await
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - `sign_and_broadcast` returns an err
    /// - cannot find the hash of the tx on chain after 60'
    /// - cannot Serialize `MsgWithdrawDelegatorReward`
    pub async fn claim_rewards(
        &mut self,
        to: &str,
        memo: Option<&str>,
    ) -> Result<GetTxResponse, CosmosClient> {
        let signer = self.signer()?;

        let mut payload = Cosmos::build().add_msg(
            MsgWithdrawDelegatorReward {
                delegator_address: signer.public_address.to_string(),
                validator_address: to.to_string(),
            }
            .to_any()?,
        );
        if let Some(memo) = memo {
            payload = payload.memo(memo);
        }

        let tx = self
            .sign_and_broadcast(payload, BroadcastMode::Sync)
            .await?;
        self.poll_for_tx(tx).await
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - `sign_and_broadcast` returns an err
    /// - cannot find the hash of the tx on chain after 60'
    /// - cannot Serialize `MsgWithdrawDelegatorReward`
    #[allow(clippy::too_many_arguments)]
    pub async fn ibc_send(
        &mut self,
        to: &str,
        coin: Coin,
        source_port: &str,
        source_channel: &str,
        timeout_height: Option<Height>,
        timeout_timestamp: u64,
        memo: Option<&str>,
    ) -> Result<GetTxResponse, CosmosClient> {
        let signer = self.signer()?;

        let mut payload = Cosmos::build().add_msg(
            MsgTransfer {
                source_port: source_port.to_string(),
                source_channel: source_channel.to_string(),
                token: Some(coin),
                sender: signer.public_address.to_string(),
                receiver: to.to_string(),
                timeout_height,
                timeout_timestamp,
            }
            .to_any()?,
        );
        if let Some(memo) = memo {
            payload = payload.memo(memo);
        }

        let tx = self
            .sign_and_broadcast(payload, BroadcastMode::Sync)
            .await?;
        self.poll_for_tx(tx).await
    }
}
