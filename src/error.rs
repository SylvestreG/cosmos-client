use prost::DecodeError;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CosmosClientError {
    #[error("Unknown cosmos-sdk Msg")]
    UnknownCosmosMsg(),
    #[error("Account does not exist {address:?}")]
    AccountDoesNotExistOnChain { address: String },
    #[error("Tendermint RPC Error")]
    TendermintRpcError(#[from] tendermint_rpc::Error),
    #[error("Decode Error")]
    ProstDecodeError(#[from] DecodeError),
    #[error("Json Error")]
    JsonError(#[from] serde_json::Error),
    #[error("CosmosRs tendermint Error")]
    CosmRsTendermintError(#[from] cosmrs::tendermint::Error),
    #[error("bip32 Error")]
    Bip32Error(#[from] cosmrs::bip32::Error),
    #[error("Timestamp Error")]
    TimestampError(#[from] prost_types::TimestampError),
    #[error("Utf8 Error")]
    Utf8Error(#[from] Utf8Error),
}
