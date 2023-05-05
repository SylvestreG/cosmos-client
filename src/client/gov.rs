use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::gov::v1beta1::{
    QueryDepositRequest, QueryDepositResponse, QueryDepositsRequest, QueryDepositsResponse,
    QueryParamsRequest, QueryParamsResponse, QueryProposalRequest, QueryProposalResponse,
    QueryProposalsRequest, QueryProposalsResponse, QueryTallyResultRequest,
    QueryTallyResultResponse, QueryVoteRequest, QueryVoteResponse, QueryVotesRequest,
    QueryVotesResponse,
};
use prost::Message;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct GovModule {
    rpc: Rc<HttpClient>,
}

impl GovModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        GovModule { rpc }
    }

    pub async fn proposal(&self, proposal_id: u64) -> Result<QueryProposalResponse, anyhow::Error> {
        let query = QueryProposalRequest { proposal_id };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.gov.v1beta1.Query/Proposal".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryProposalResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn proposals(
        &self,
        proposal_status: i32,
        voter: &str,
        depositor: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryProposalsResponse, anyhow::Error> {
        let query = QueryProposalsRequest {
            proposal_status,
            voter: voter.to_string(),
            depositor: depositor.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.gov.v1beta1.Query/Proposals".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryProposalsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn vote(
        &self,
        proposal_id: u64,
        voter: &str,
    ) -> Result<QueryVoteResponse, anyhow::Error> {
        let query = QueryVoteRequest {
            proposal_id,
            voter: voter.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.gov.v1beta1.Query/Vote".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryVoteResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn votes(
        &self,
        proposal_id: u64,
        pagination: Option<PageRequest>,
    ) -> Result<QueryVotesResponse, anyhow::Error> {
        let query = QueryVotesRequest {
            proposal_id,
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.gov.v1beta1.Query/Votes".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryVotesResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn params(&self, params_type: &str) -> Result<QueryParamsResponse, anyhow::Error> {
        let query = QueryParamsRequest {
            params_type: params_type.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.gov.v1beta1.Query/Params".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryParamsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn deposit(
        &self,
        proposal_id: u64,
        depositor: &str,
    ) -> Result<QueryDepositResponse, anyhow::Error> {
        let query = QueryDepositRequest {
            proposal_id,
            depositor: depositor.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.gov.v1beta1.Query/Deposit".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryDepositResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn deposits(
        &self,
        proposal_id: u64,
        pagination: Option<PageRequest>,
    ) -> Result<QueryDepositsResponse, anyhow::Error> {
        let query = QueryDepositsRequest {
            proposal_id,
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.gov.v1beta1.Query/Deposits".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryDepositsResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn tally_result(
        &self,
        proposal_id: u64,
    ) -> Result<QueryTallyResultResponse, anyhow::Error> {
        let query = QueryTallyResultRequest { proposal_id };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.gov.v1beta1.Query/TallyResult".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryTallyResultResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }
}
