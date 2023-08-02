use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::authz::v1beta1::{
    QueryGranteeGrantsRequest, QueryGranteeGrantsResponse, QueryGranterGrantsRequest,
    QueryGranterGrantsResponse, QueryGrantsRequest, QueryGrantsResponse,
};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
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
    #[allow(clippy::similar_names)]
    pub async fn grants(
        &self,
        granter: &str,
        grantee: &str,
        msg_type_url: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryGrantsResponse, CosmosClient> {
        let query = QueryGrantsRequest {
            granter: granter.to_string(),
            grantee: grantee.to_string(),
            msg_type_url: msg_type_url.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.authz.v1beta1.Query/Grants".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryGrantsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn granter_grants(
        &self,
        granter: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryGranterGrantsResponse, CosmosClient> {
        let query = QueryGranterGrantsRequest {
            granter: granter.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.authz.v1beta1.Query/GranterGrants".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryGranterGrantsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn grantee_grants(
        &self,
        grantee: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryGranteeGrantsResponse, CosmosClient> {
        let query = QueryGranteeGrantsRequest {
            grantee: grantee.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.authz.v1beta1.Query/GranteeGrants".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryGranteeGrantsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
