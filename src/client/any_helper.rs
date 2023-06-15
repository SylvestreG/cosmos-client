use crate::error::CosmosClientError;
use cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount;
use cosmos_sdk_proto::cosmos::crypto::ed25519::PubKey;
use cosmos_sdk_proto::cosmos::evidence::v1beta1::Equivocation;
use cosmos_sdk_proto::cosmos::feegrant::v1beta1::BasicAllowance;
use cosmos_sdk_proto::Any;
use cosmrs::bank::MsgSend;
use prost::Message;

#[derive(Debug)]
pub enum CosmosType {
    BaseAccount(BaseAccount),
    PubKey(PubKey),
    Equivocation(Equivocation),
    BasicAllowance(BasicAllowance),
    MsgSend(MsgSend),
}

pub fn any_to_cosmos(cosmos: &Any) -> Result<CosmosType, CosmosClientError> {
    match cosmos.type_url.as_str() {
        "/cosmos.auth.v1beta1.BaseAccount" => Ok(CosmosType::BaseAccount(BaseAccount::decode(
            cosmos.value.as_slice(),
        )?)),
        "/cosmos.crypto.secp256k1.PubKey" => {
            Ok(CosmosType::PubKey(PubKey::decode(cosmos.value.as_slice())?))
        }
        "/cosmos.evidence.v1beta1.Equivocation" => Ok(CosmosType::Equivocation(
            Equivocation::decode(cosmos.value.as_slice())?,
        )),
        "/cosmos.feegrant.v1beta1.BasicAllowance" => Ok(CosmosType::BasicAllowance(
            BasicAllowance::decode(cosmos.value.as_slice())?,
        )),
        _ => Err(CosmosClientError::UnknownCosmosMsg()),
    }
}
