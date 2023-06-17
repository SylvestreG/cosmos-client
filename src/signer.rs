use crate::error::CosmosClientError;
use cosmrs::bip32::{Language, Mnemonic, XPrv};
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::crypto::PublicKey;
use cosmrs::AccountId;
use hex::decode;
use rand_core::OsRng;

pub struct Signer {
    pub mnemonic: Option<String>,
    pub denom: String,
    pub public_address: AccountId,
    pub private_key: SigningKey,
    pub public_key: PublicKey,
    pub gas_adjustment_percent: u8,
    pub gas_price: u128,
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
        let public_address = public_key.account_id(prefix)?;

        Ok((private_key, public_key, public_address))
    }

    pub fn generate(
        prefix: &str,
        denom: &str,
        derivation: Option<&str>,
        gas_adjustment_percent: u8,
        gas_price: u128,
    ) -> Result<Self, CosmosClientError> {
        let mnemonic = Mnemonic::random(OsRng, Language::English);
        let (private_key, public_key, public_address) =
            Signer::load_from_mnemonic(mnemonic.phrase(), prefix, derivation)?;

        Ok(Signer {
            mnemonic: Some(mnemonic.phrase().to_string()),
            public_address,
            gas_adjustment_percent,
            gas_price,
            denom: denom.to_string(),
            private_key,
            public_key,
        })
    }

    pub fn from_pkey(
        private_key: &str,
        prefix: &str,
        denom: &str,
        gas_adjustment_percent: u8,
        gas_price: u128,
    ) -> Result<Self, CosmosClientError> {
        let private_key = SigningKey::from_slice(decode(private_key)?.as_slice())?;
        let public_key = private_key.public_key();
        let public_address = public_key.account_id(prefix)?;

        Ok(Signer {
            mnemonic: None,
            public_address,
            gas_adjustment_percent,
            gas_price,
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
        gas_adjustment_percent: u8,
        gas_price: u128,
    ) -> Result<Self, CosmosClientError> {
        let (private_key, public_key, public_address) =
            Signer::load_from_mnemonic(phrase, prefix, derivation)?;

        Ok(Signer {
            mnemonic: Some(phrase.to_string()),
            public_address,
            gas_adjustment_percent,
            gas_price,
            denom: denom.to_string(),
            private_key,
            public_key,
        })
    }
}
