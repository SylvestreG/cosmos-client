use cosmos_sdk_proto::cosmos::mint::v1beta1::{
    QueryAnnualProvisionsRequest, QueryAnnualProvisionsResponse, QueryInflationRequest,
    QueryInflationResponse, QueryParamsRequest, QueryParamsResponse,
};
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

    pub async fn annual_provisions(&self) -> Result<QueryAnnualProvisionsResponse, anyhow::Error> {
        let query = QueryAnnualProvisionsRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.mint.v1beta1.Query/AnnualProvisions".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryAnnualProvisionsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn inflation(&self) -> Result<QueryInflationResponse, anyhow::Error> {
        let query = QueryInflationRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.mint.v1beta1.Query/Inflation".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryInflationResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn params(&self) -> Result<QueryParamsResponse, anyhow::Error> {
        let query = QueryParamsRequest {};
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.mint.v1beta1.Query/Params".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryParamsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }
}
