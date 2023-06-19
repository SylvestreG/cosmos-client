use crate::error::CosmosClientError;
use crate::error::CosmosClientError::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::mint::v1beta1::{
    QueryAnnualProvisionsRequest, QueryAnnualProvisionsResponse, QueryInflationRequest,
    QueryInflationResponse, QueryParamsRequest, QueryParamsResponse,
};
use cosmrs::tendermint::abci::Code;
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct MintModule {
    rpc: Rc<HttpClient>,
}

impl MintModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        MintModule { rpc }
    }

    pub async fn annual_provisions(
        &self,
    ) -> Result<QueryAnnualProvisionsResponse, CosmosClientError> {
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

    pub async fn inflation(&self) -> Result<QueryInflationResponse, CosmosClientError> {
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

    pub async fn params(&self) -> Result<QueryParamsResponse, CosmosClientError> {
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
