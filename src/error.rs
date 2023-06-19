use cosmrs::ErrorReport;
use hex::FromHexError;
use prost::{DecodeError, EncodeError};
use std::convert::Infallible;
use std::io;
use std::num::ParseIntError;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CosmosClientError {
    #[error("Tendermint RPC Error")]
    TendermintRpcError(#[from] tendermint_rpc::Error),
    #[error("Decode Error")]
    ProstDecodeError(#[from] DecodeError),
    #[error("Decode Error")]
    ProstEncodeError(#[from] EncodeError),
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
    #[error("Infaillible")]
    Infaillible(#[from] Infallible),
    #[error("Parse Int Error")]
    ParseIntError(#[from] ParseIntError),

    #[error("Unknown cosmos-sdk Msg")]
    UnknownCosmosMsg,
    #[error("Account does not exist {address:?}")]
    AccountDoesNotExistOnChain { address: String },
    #[error("Cannot simulate TX Gas Fee")]
    CannotSimulateTxGasFee,
    #[error("No signer attached")]
    NoSignerAttached,
    #[error("Rpc errors : {0}")]
    RpcError(String),
    #[error("Tx Polling Timeout")]
    TXPollingTimeout,
}
