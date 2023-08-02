use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::evidence::v1beta1::{
    QueryAllEvidenceRequest, QueryAllEvidenceResponse, QueryEvidenceRequest, QueryEvidenceResponse,
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
    pub async fn evidence(
        &self,
        evidence_hash: Vec<u8>,
    ) -> Result<QueryEvidenceResponse, CosmosClient> {
        let query = QueryEvidenceRequest { evidence_hash };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.evidence.v1beta1.Query/Evidence".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryEvidenceResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn all_evidence(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAllEvidenceResponse, CosmosClient> {
        let query = QueryAllEvidenceRequest { pagination };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.evidence.v1beta1.Query/AllEvidence".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryAllEvidenceResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
