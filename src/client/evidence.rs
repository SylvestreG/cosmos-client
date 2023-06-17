use crate::error::CosmosClientError;
use crate::error::CosmosClientError::ProstDecodeError;
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::evidence::v1beta1::{
    QueryAllEvidenceRequest, QueryAllEvidenceResponse, QueryEvidenceRequest, QueryEvidenceResponse,
};
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct EvidenceModule {
    rpc: Rc<HttpClient>,
}

impl EvidenceModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        EvidenceModule { rpc }
    }

    pub async fn evidence(
        &self,
        evidence_hash: Vec<u8>,
    ) -> Result<QueryEvidenceResponse, CosmosClientError> {
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

        QueryEvidenceResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn all_evidence(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAllEvidenceResponse, CosmosClientError> {
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

        QueryAllEvidenceResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
