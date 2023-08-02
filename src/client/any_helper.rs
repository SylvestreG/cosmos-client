use crate::error::CosmosClient;
use cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount;
use cosmos_sdk_proto::cosmos::crypto::ed25519::PubKey;
use cosmos_sdk_proto::cosmos::evidence::v1beta1::Equivocation;
use cosmos_sdk_proto::cosmos::feegrant::v1beta1::BasicAllowance;
use cosmos_sdk_proto::cosmos::vesting::v1beta1::ContinuousVestingAccount;
use cosmos_sdk_proto::Any;
use cosmrs::bank::MsgSend;
use prost::Message;

#[derive(Debug)]
pub enum CosmosType {
    BaseAccount(BaseAccount),
    ContinuousVestingAccount(ContinuousVestingAccount),
    PubKey(PubKey),
    Equivocation(Equivocation),
    BasicAllowance(BasicAllowance),
    MsgSend(MsgSend),
}

/// # Errors
///
/// Will return `Err` if `cosmos` is an unknown cosmos msg is given or if
/// a decode fail
pub fn any_to_cosmos(cosmos: &Any) -> Result<CosmosType, CosmosClient> {
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
        "/cosmos.vesting.v1beta1.ContinuousVestingAccount" => {
            Ok(CosmosType::ContinuousVestingAccount(
                ContinuousVestingAccount::decode(cosmos.value.as_slice())?,
            ))
        }
        _ => Err(CosmosClient::UnknownCosmosMsg),
    }
}
