use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::params::v1beta1::{QueryParamsRequest, QueryParamsResponse};
use cosmrs::tendermint::abci::Code;
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

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
    pub async fn params(
        &self,
        subspace: &str,
        key: &str,
    ) -> Result<QueryParamsResponse, CosmosClient> {
        let query = QueryParamsRequest {
            subspace: subspace.to_string(),
            key: key.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.params.v1beta1.Query/Params".to_string()),
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
