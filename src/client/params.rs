use cosmos_sdk_proto::cosmos::params::v1beta1::{QueryParamsRequest, QueryParamsResponse};
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct ParamsModule {
    rpc: Rc<HttpClient>,
}

impl ParamsModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        ParamsModule { rpc }
    }

    pub async fn params(
        &self,
        subspace: &str,
        key: &str,
    ) -> Result<QueryParamsResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryParamsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }
}
