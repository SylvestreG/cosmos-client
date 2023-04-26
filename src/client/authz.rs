use cosmos_sdk_proto::cosmos::authz::v1beta1::{
    QueryGranteeGrantsRequest, QueryGranteeGrantsResponse, QueryGranterGrantsRequest,
    QueryGranterGrantsResponse, QueryGrantsRequest, QueryGrantsResponse,
};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct AuthzModule {
    rpc: Rc<HttpClient>,
}

impl AuthzModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        AuthzModule { rpc }
    }

    pub async fn grants(
        &self,
        granter: &str,
        grantee: &str,
        msg_type_url: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryGrantsResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryGrantsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn granter_grants(
        &self,
        granter: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryGranterGrantsResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryGranterGrantsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn grantee_grants(
        &self,
        grantee: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryGranteeGrantsResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryGranteeGrantsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }
}
