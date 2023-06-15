use crate::error::CosmosClientError;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{SimulateRequest, SimulateResponse};
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct TxModule {
    rpc: Rc<HttpClient>,
}

impl TxModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        TxModule { rpc }
    }

    pub async fn simulate(&self, payload: Vec<u8>) -> Result<SimulateResponse, CosmosClientError> {
        #[allow(deprecated)]
        let query = SimulateRequest {
            tx: None,
            tx_bytes: payload,
        };
        println!("in {:#?}", query);
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.tx.v1beta1.Service/Simulate".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = SimulateResponse::decode(query.value.as_slice())?;
        println!("out {:#?}", resp);
        Ok(resp)
    }
}
