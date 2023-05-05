use cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount;
use cosmos_sdk_proto::cosmos::crypto::ed25519::PubKey;
use cosmos_sdk_proto::cosmos::evidence::v1beta1::Equivocation;
use cosmos_sdk_proto::Any;
use prost::Message;

#[derive(Debug)]
pub enum CosmosType {
    BaseAccount(BaseAccount),
    PubKey(PubKey),
    Equivocation(Equivocation),
    Unknown {},
}

pub fn any_to_cosmos(cosmos: &Any) -> Result<CosmosType, anyhow::Error> {
    Ok(match cosmos.type_url.as_str() {
        "/cosmos.auth.v1beta1.BaseAccount" => {
            CosmosType::BaseAccount(BaseAccount::decode(cosmos.value.as_slice())?)
        }
        "/cosmos.crypto.secp256k1.PubKey" => {
            CosmosType::PubKey(PubKey::decode(cosmos.value.as_slice())?)
        }
        "/cosmos.evidence.v1beta1.Equivocation" => {
            CosmosType::Equivocation(Equivocation::decode(cosmos.value.as_slice())?)
        }
        _ => CosmosType::Unknown {},
    })
}
