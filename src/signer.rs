use crate::client::any_helper::{any_to_cosmos, CosmosType};
use crate::client::RpcClient;
use crate::error::CosmosClientError;
use cosmrs::bip32::{Language, Mnemonic, XPrv};
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::crypto::PublicKey;
use cosmrs::tendermint::chain;
use cosmrs::tx::{BodyBuilder, SignDoc, SignerInfo};
use cosmrs::{tx, AccountId, Coin};
use hex::decode;
use prost_types::Any;
use rand_core::OsRng;
use std::str::FromStr;

pub struct Signer {
    mnemonic: Option<String>,
    denom: String,
    public_address: AccountId,
    private_key: SigningKey,
    public_key: PublicKey,
}

pub struct CosmosTx {
    tx: BodyBuilder,
}

impl CosmosTx {
    pub fn build() -> Self {
        CosmosTx {
            tx: BodyBuilder::new(),
        }
    }

    pub fn memo(mut self, memo: &str) -> Self {
        self.tx.memo(memo.to_string());
        self
    }

    pub fn add_msg(mut self, msg: Any) -> Self {
        self.tx.msg(msg);
        self
    }

    pub async fn sign(
        &mut self,
        signer: Signer,
        mut client: RpcClient,
    ) -> Result<Vec<u8>, CosmosClientError> {
        let account = client
            .auth
            .account(signer.public_address.to_string().as_str())
            .await?;

        if let Some(account) = account.account {
            if let Ok(CosmosType::BaseAccount(account)) = any_to_cosmos(&account) {
                let tx_body = self.tx.finish();
                let auth_info =
                    SignerInfo::single_direct(Some(signer.public_key), account.sequence).auth_info(
                        tx::Fee::from_amount_and_gas(
                            Coin {
                                amount: 2500u128,
                                denom: signer.denom.parse().unwrap(),
                            },
                            100u64,
                        ),
                    );
                let sign_doc = SignDoc::new(
                    &tx_body,
                    &auth_info,
                    &chain::Id::from_str(client.chain_id().await?.as_str())?,
                    account.account_number,
                )
                .unwrap();
                let tx_raw = sign_doc.sign(&signer.private_key).unwrap();
                let _tx = client.tx.simulate(tx_raw.to_bytes().unwrap()).await?;
            }
        } else {
            return Err(CosmosClientError::AccountDoesNotExistOnChain {
                address: signer.get_address(),
            });
        }

        Ok(vec![])
    }
}

impl Signer {
    fn load_from_mnemonic(
        phrase: &str,
        prefix: &str,
        derivation: Option<&str>,
    ) -> Result<(SigningKey, PublicKey, AccountId), CosmosClientError> {
        let derivation = if let Some(derivation) = derivation {
            derivation
        } else {
            "m/44'/118'/0'/0/0"
        };

        let mnemonic = Mnemonic::new(phrase, Language::English)?;
        let pri = XPrv::derive_from_path(&mnemonic.to_seed(""), &derivation.parse()?)?;
        let private_key = SigningKey::from(pri);
        let public_key = private_key.public_key();
        let public_address = public_key.account_id(prefix).unwrap();

        Ok((private_key, public_key, public_address))
    }

    pub fn generate(
        prefix: &str,
        denom: &str,
        derivation: Option<&str>,
    ) -> Result<Self, CosmosClientError> {
        let mnemonic = Mnemonic::random(OsRng, Language::English);
        let (private_key, public_key, public_address) =
            Signer::load_from_mnemonic(mnemonic.phrase(), prefix, derivation)?;

        Ok(Signer {
            mnemonic: Some(mnemonic.phrase().to_string()),
            public_address,
            denom: denom.to_string(),
            private_key,
            public_key,
        })
    }

    pub fn from_pkey(
        private_key: &str,
        prefix: &str,
        denom: &str,
    ) -> Result<Self, CosmosClientError> {
        let private_key = SigningKey::from_slice(decode(private_key)?.as_slice())?;
        let public_key = private_key.public_key();
        let public_address = public_key.account_id(prefix).unwrap();

        Ok(Signer {
            mnemonic: None,
            public_address,
            denom: denom.to_string(),
            private_key,
            public_key,
        })
    }

    pub fn from_mnemonic(
        phrase: &str,
        prefix: &str,
        denom: &str,
        derivation: Option<&str>,
    ) -> Result<Self, CosmosClientError> {
        let (private_key, public_key, public_address) =
            Signer::load_from_mnemonic(phrase, prefix, derivation)?;

        Ok(Signer {
            mnemonic: Some(phrase.to_string()),
            public_address,
            denom: denom.to_string(),
            private_key,
            public_key,
        })
    }

    pub fn get_address(&self) -> String {
        self.public_address.to_string()
    }

    pub fn get_phrase(&self) -> Option<String> {
        self.mnemonic.clone()
    }

    pub fn get_denom(&self) -> String {
        self.denom.clone()
    }
}
