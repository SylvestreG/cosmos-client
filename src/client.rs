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
use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward;
use cosmos_sdk_proto::cosmos::staking::v1beta1::{MsgDelegate, MsgUndelegate};
use cosmos_sdk_proto::cosmos::tx::v1beta1::{BroadcastMode, GetTxResponse};
use cosmos_sdk_proto::traits::MessageExt;
use cosmrs::tendermint::chain;
use cosmrs::tx::{Fee, SignDoc, SignerInfo};
use std::ops::{DivAssign, MulAssign};
use std::rc::Rc;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;
use tendermint_rpc::{Client, HttpClient};

use crate::client::auth::AuthModule;
use crate::client::authz::AuthzModule;
use crate::client::bank::BankModule;
use crate::client::distribution::DistributionModule;
use crate::client::evidence::EvidenceModule;
use crate::client::feegrant::FeeGrantModule;
use crate::client::gov::GovModule;
use crate::client::mint::MintModule;
use crate::client::params::ParamsModule;
use crate::client::slashing::SlashingModule;
use crate::client::staking::StakingModule;
use crate::client::tx::{TxModule, TxResponse};
use crate::client::upgrade::UpgradeModule;
use crate::client::wasm::WasmModule;
use crate::error::CosmosClientError;
use crate::error::CosmosClientError::{AccountDoesNotExistOnChain, NoSignerAttached};
use crate::signer::Signer;
use crate::tx::CosmosTx;

pub struct RpcClient {
    chain_id: String,
    signer: Option<Signer>,
    account_id: Option<u64>,
    sequence_id: Option<u64>,
    pub bank: BankModule,
    pub auth: AuthModule,
    pub authz: AuthzModule,
    pub distribution: DistributionModule,
    pub evidence: EvidenceModule,
    pub feegrant: FeeGrantModule,
    pub gov: GovModule,
    pub mint: MintModule,
    pub params: ParamsModule,
    pub slashing: SlashingModule,
    pub staking: StakingModule,
    pub tx: TxModule,
    pub upgrade: UpgradeModule,
    pub wasm: WasmModule,
}

impl RpcClient {
    pub async fn new(url: &str) -> Result<Self, CosmosClientError> {
        let rpc = Rc::new(HttpClient::new(url)?);

        Ok(RpcClient {
            chain_id: rpc.status().await?.node_info.network.to_string(),
            signer: None,
            account_id: None,
            sequence_id: None,
            auth: AuthModule::new(rpc.clone()),
            authz: AuthzModule::new(rpc.clone()),
            bank: BankModule::new(rpc.clone()),
            distribution: DistributionModule::new(rpc.clone()),
            evidence: EvidenceModule::new(rpc.clone()),
            feegrant: FeeGrantModule::new(rpc.clone()),
            gov: GovModule::new(rpc.clone()),
            mint: MintModule::new(rpc.clone()),
            params: ParamsModule::new(rpc.clone()),
            slashing: SlashingModule::new(rpc.clone()),
            staking: StakingModule::new(rpc.clone()),
            tx: TxModule::new(rpc.clone()),
            upgrade: UpgradeModule::new(rpc.clone()),
            wasm: WasmModule::new(rpc),
        })
    }

    pub async fn attach_signer(&mut self, signer: Signer) -> Result<(), CosmosClientError> {
        self.signer = Some(signer);
        self.update_sequence_id().await?;
        Ok(())
    }

    pub async fn update_sequence_id(&mut self) -> Result<(), CosmosClientError> {
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

    pub async fn sign(&mut self, tx: CosmosTx) -> Result<Vec<u8>, CosmosClientError> {
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
            return Err(CosmosClientError::CannotSimulateTxGasFee);
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

    pub async fn broadcast(
        &mut self,
        payload: Vec<u8>,
        mode: BroadcastMode,
    ) -> Result<TxResponse, CosmosClientError> {
        self.tx.broadcast(payload, mode).await
    }

    pub async fn sign_and_broadcast(
        &mut self,
        tx: CosmosTx,
        mode: BroadcastMode,
    ) -> Result<TxResponse, CosmosClientError> {
        let payload = self.sign(tx).await?;

        self.tx.broadcast(payload, mode).await
    }

    fn signer(&self) -> Result<&Signer, CosmosClientError> {
        self.signer.as_ref().ok_or(NoSignerAttached)
    }

    async fn poll_for_tx(&self, tx: TxResponse) -> Result<GetTxResponse, CosmosClientError> {
        let hash = match tx {
            TxResponse::Sync(tx) => tx.hash,
            TxResponse::Async(tx) => tx.hash,
            TxResponse::Commit(tx) => tx.hash,
        };

        for _ in 0..60 {
            let tx = self.tx.get_tx(hash.to_string().as_str()).await;

            if tx.is_ok() {
                return tx;
            }
            sleep(Duration::from_secs(3));
        }

        Err(CosmosClientError::TXPollingTimeout)
    }

    pub async fn send(
        &mut self,
        to: &str,
        coin: Vec<Coin>,
        memo: Option<&str>,
    ) -> Result<GetTxResponse, CosmosClientError> {
        let signer = self.signer()?;

        let mut payload = CosmosTx::build().add_msg(
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

    pub async fn stake(
        &mut self,
        to: &str,
        coin: Coin,
        memo: Option<&str>,
    ) -> Result<GetTxResponse, CosmosClientError> {
        let signer = self.signer()?;

        let mut payload = CosmosTx::build().add_msg(
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

    pub async fn unstake(
        &mut self,
        to: &str,
        coin: Coin,
        memo: Option<&str>,
    ) -> Result<GetTxResponse, CosmosClientError> {
        let signer = self.signer()?;

        let mut payload = CosmosTx::build().add_msg(
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

    pub async fn claim_rewards(
        &mut self,
        to: &str,
        memo: Option<&str>,
    ) -> Result<GetTxResponse, CosmosClientError> {
        let signer = self.signer()?;

        let mut payload = CosmosTx::build().add_msg(
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
}
