use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::slashing::v1beta1::{
    QueryParamsRequest, QueryParamsResponse, QuerySigningInfoRequest, QuerySigningInfoResponse,
    QuerySigningInfosRequest, QuerySigningInfosResponse,
};
use tendermint::abci::Code;
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
    pub async fn params(&self) -> Result<QueryParamsResponse, CosmosClient> {
        let query = QueryParamsRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.slashing.v1beta1.Query/Params".to_string()),
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

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn signing_info(
        &self,
        cons_address: &str,
    ) -> Result<QuerySigningInfoResponse, CosmosClient> {
        let query = QuerySigningInfoRequest {
            cons_address: cons_address.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.slashing.v1beta1.Query/SigningInfo".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QuerySigningInfoResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn signing_infos(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QuerySigningInfosResponse, CosmosClient> {
        let query = QuerySigningInfosRequest { pagination };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.slashing.v1beta1.Query/SigningInfos".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QuerySigningInfosResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
