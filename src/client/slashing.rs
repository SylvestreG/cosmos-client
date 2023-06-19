use crate::error::CosmosClientError;
use crate::error::CosmosClientError::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::slashing::v1beta1::{
    QueryParamsRequest, QueryParamsResponse, QuerySigningInfoRequest, QuerySigningInfoResponse,
    QuerySigningInfosRequest, QuerySigningInfosResponse,
};
use cosmrs::tendermint::abci::Code;
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct SlashingModule {
    rpc: Rc<HttpClient>,
}

impl SlashingModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        SlashingModule { rpc }
    }

    pub async fn params(&self) -> Result<QueryParamsResponse, CosmosClientError> {
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

    pub async fn signing_info(
        &self,
        cons_address: &str,
    ) -> Result<QuerySigningInfoResponse, CosmosClientError> {
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

    pub async fn signing_infos(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QuerySigningInfosResponse, CosmosClientError> {
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
