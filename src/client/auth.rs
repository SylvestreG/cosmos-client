use crate::error::CosmosClientError;
use crate::error::CosmosClientError::ProstDecodeError;
use cosmos_sdk_proto::cosmos::auth::v1beta1::{
    QueryAccountRequest, QueryAccountResponse, QueryAccountsRequest, QueryAccountsResponse,
    QueryParamsRequest, QueryParamsResponse,
};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct AuthModule {
    rpc: Rc<HttpClient>,
}

impl AuthModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        AuthModule { rpc }
    }

    pub async fn accounts(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAccountsResponse, CosmosClientError> {
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

        QueryAccountsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn account(&self, address: &str) -> Result<QueryAccountResponse, CosmosClientError> {
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

        QueryAccountResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn params(&self) -> Result<QueryParamsResponse, CosmosClientError> {
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

        QueryParamsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
