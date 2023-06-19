use crate::error::CosmosClientError;
use crate::error::CosmosClientError::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::feegrant::v1beta1::{
    QueryAllowanceRequest, QueryAllowanceResponse, QueryAllowancesRequest, QueryAllowancesResponse,
};
use cosmrs::tendermint::abci::Code;
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct FeeGrantModule {
    rpc: Rc<HttpClient>,
}

impl FeeGrantModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        FeeGrantModule { rpc }
    }

    pub async fn allowance(
        &self,
        granter: &str,
        grantee: &str,
    ) -> Result<QueryAllowanceResponse, CosmosClientError> {
        let query = QueryAllowanceRequest {
            granter: granter.to_string(),
            grantee: grantee.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.feegrant.v1beta1.Query/Allowance".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryAllowanceResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn allowances(
        &self,
        grantee: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAllowancesResponse, CosmosClientError> {
        let query = QueryAllowancesRequest {
            grantee: grantee.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.feegrant.v1beta1.Query/Allowances".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryAllowancesResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
