use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::auth::v1beta1::{
    QueryAccountRequest, QueryAccountResponse, QueryAccountsRequest, QueryAccountsResponse,
    QueryParamsRequest, QueryParamsResponse,
};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use prost::Message;
use std::sync::Arc;
use tendermint::abci::Code;
use tendermint_rpc::{Client, HttpClient};

pub struct Module {
    rpc: Arc<HttpClient>,
}

impl Module {
    pub fn new(rpc: Arc<HttpClient>) -> Self {
        Module { rpc }
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn accounts(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAccountsResponse, CosmosClient> {
        let query = QueryAccountsRequest { pagination };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.auth.v1beta1.Query/Accounts".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryAccountsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn account(&self, address: &str) -> Result<QueryAccountResponse, CosmosClient> {
        let query = QueryAccountRequest {
            address: address.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.auth.v1beta1.Query/Account".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryAccountResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn params(&self) -> Result<QueryParamsResponse, CosmosClient> {
        let query = QueryParamsRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.auth.v1beta1.Query/Params".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryParamsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
