use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::tx::v1beta1::{
    BroadcastMode, GetTxRequest, GetTxResponse, SimulateRequest, SimulateResponse,
};
use prost::Message;
use std::rc::Rc;
use tendermint::abci::Code;
use tendermint_rpc::endpoint::broadcast::{tx_async, tx_commit, tx_sync};
use tendermint_rpc::{Client, HttpClient};

#[derive(Clone, Debug)]
pub enum Response {
    Async(tx_async::Response),
    Sync(tx_sync::Response),
    Commit(tx_commit::Response),
}

pub struct Module {
    rpc: Rc<HttpClient>,
}

impl Module {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        Module { rpc }
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn simulate(&self, payload: Vec<u8>) -> Result<SimulateResponse, CosmosClient> {
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

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn broadcast(
        &self,
        payload: Vec<u8>,
        mode: BroadcastMode,
    ) -> Result<Response, CosmosClient> {
        Ok(match mode {
            BroadcastMode::Async | BroadcastMode::Unspecified => {
                Response::Async(self.rpc.broadcast_tx_async(payload).await?)
            }
            BroadcastMode::Sync => Response::Sync(self.rpc.broadcast_tx_sync(payload).await?),
            BroadcastMode::Block => Response::Commit(self.rpc.broadcast_tx_commit(payload).await?),
        })
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn get_tx(&self, hash: &str) -> Result<GetTxResponse, CosmosClient> {
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
