use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::mint::v1beta1::{
    QueryAnnualProvisionsRequest, QueryAnnualProvisionsResponse, QueryInflationRequest,
    QueryInflationResponse, QueryParamsRequest, QueryParamsResponse,
};
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
    pub async fn annual_provisions(&self) -> Result<QueryAnnualProvisionsResponse, CosmosClient> {
        let query = QueryAnnualProvisionsRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.mint.v1beta1.Query/AnnualProvisions".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryAnnualProvisionsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn inflation(&self) -> Result<QueryInflationResponse, CosmosClient> {
        let query = QueryInflationRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.mint.v1beta1.Query/Inflation".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryInflationResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
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
                Some("/cosmos.mint.v1beta1.Query/Params".to_string()),
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
