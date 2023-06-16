use cosmrs::ErrorReport;
use hex::FromHexError;
use prost::DecodeError;
use std::io;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CosmosClientError {
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
    #[error("Utf8 Error")]
    IoError(#[from] io::Error),
    #[error("ErrorReport")]
    ErrorReport(#[from] ErrorReport),
    #[error("FromHexError")]
    FromHexError(#[from] FromHexError),

    #[error("Unknown cosmos-sdk Msg")]
    UnknownCosmosMsg(),
    #[error("Account does not exist {address:?}")]
    AccountDoesNotExistOnChain { address: String },
}
