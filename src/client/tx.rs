use crate::error::CosmosClientError;
use crate::error::CosmosClientError::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::tx::v1beta1::{
    BroadcastMode, GetTxRequest, GetTxResponse, SimulateRequest, SimulateResponse,
};
use cosmrs::tendermint::abci::Code;
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::endpoint::broadcast::{tx_async, tx_commit, tx_sync};
use tendermint_rpc::{Client, HttpClient};

#[derive(Clone, Debug)]
pub enum TxResponse {
    Async(tx_async::Response),
    Sync(tx_sync::Response),
    Commit(tx_commit::Response),
}

pub struct TxModule {
    rpc: Rc<HttpClient>,
}

impl TxModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        TxModule { rpc }
    }

    pub async fn simulate(&self, payload: Vec<u8>) -> Result<SimulateResponse, CosmosClientError> {
        #[allow(deprecated)]
        let query = SimulateRequest {
            tx: None,
            tx_bytes: payload,
        };

        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.tx.v1beta1.Service/Simulate".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        SimulateResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn broadcast(
        &self,
        payload: Vec<u8>,
        mode: BroadcastMode,
    ) -> Result<TxResponse, CosmosClientError> {
        Ok(match mode {
            BroadcastMode::Async | BroadcastMode::Unspecified => {
                TxResponse::Async(self.rpc.broadcast_tx_async(payload).await?)
            }
            BroadcastMode::Sync => TxResponse::Sync(self.rpc.broadcast_tx_sync(payload).await?),
            BroadcastMode::Block => {
                TxResponse::Commit(self.rpc.broadcast_tx_commit(payload).await?)
            }
        })
    }

    pub async fn get_tx(&self, hash: &str) -> Result<GetTxResponse, CosmosClientError> {
        let query = GetTxRequest {
            hash: hash.to_string(),
        };

        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.tx.v1beta1.Service/GetTx".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        GetTxResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
