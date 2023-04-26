use cosmrs::bip32::{Language, Mnemonic, XPrv};
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::crypto::PublicKey;
use cosmrs::AccountId;
use rand_core::OsRng;

pub struct Signer {
    mnemonic: Option<String>,
    public_address: AccountId,
    _private_key: SigningKey,
    _public_key: PublicKey,
}

impl Signer {
    fn load_from_mnemonic(
        phrase: &str,
        prefix: &str,
        derivation: Option<&str>,
    ) -> Result<(SigningKey, PublicKey, AccountId), anyhow::Error> {
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
        _denom: &str,
        derivation: Option<&str>,
    ) -> Result<Self, anyhow::Error> {
        let mnemonic = Mnemonic::random(OsRng, Language::English);
        let (_private_key, _public_key, public_address) =
            Signer::load_from_mnemonic(mnemonic.phrase(), prefix, derivation)?;

        Ok(Signer {
            mnemonic: Some(mnemonic.phrase().to_string()),
            public_address,
            _private_key,
            _public_key,
        })
    }

    pub fn from_mnemonic(
        phrase: &str,
        prefix: &str,
        _denom: &str,
        derivation: Option<&str>,
    ) -> Result<Self, anyhow::Error> {
        let (_private_key, _public_key, public_address) =
            Signer::load_from_mnemonic(phrase, prefix, derivation)?;

        Ok(Signer {
            mnemonic: Some(phrase.to_string()),
            public_address,
            _private_key,
            _public_key,
        })
    }

    pub fn get_address(&self) -> String {
        self.public_address.to_string()
    }

    pub fn get_phrase(&self) -> Option<String> {
        self.mnemonic.clone()
    }
}
