use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmos::gov::v1beta1::{
    QueryDepositRequest, QueryDepositResponse, QueryDepositsRequest, QueryDepositsResponse,
    QueryParamsRequest, QueryParamsResponse, QueryProposalRequest, QueryProposalResponse,
    QueryProposalsRequest, QueryProposalsResponse, QueryTallyResultRequest,
    QueryTallyResultResponse, QueryVoteRequest, QueryVoteResponse, QueryVotesRequest,
    QueryVotesResponse,
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
    pub async fn proposal(&self, proposal_id: u64) -> Result<QueryProposalResponse, CosmosClient> {
        let query = QueryProposalRequest { proposal_id };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.gov.v1beta1.Query/Proposal".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryProposalResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn proposals(
        &self,
        proposal_status: i32,
        voter: &str,
        depositor: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryProposalsResponse, CosmosClient> {
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
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryProposalsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn vote(
        &self,
        proposal_id: u64,
        voter: &str,
    ) -> Result<QueryVoteResponse, CosmosClient> {
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
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryVoteResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn votes(
        &self,
        proposal_id: u64,
        pagination: Option<PageRequest>,
    ) -> Result<QueryVotesResponse, CosmosClient> {
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
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryVotesResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn params(&self, params_type: &str) -> Result<QueryParamsResponse, CosmosClient> {
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
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryParamsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn deposit(
        &self,
        proposal_id: u64,
        depositor: &str,
    ) -> Result<QueryDepositResponse, CosmosClient> {
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
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDepositResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn deposits(
        &self,
        proposal_id: u64,
        pagination: Option<PageRequest>,
    ) -> Result<QueryDepositsResponse, CosmosClient> {
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
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryDepositsResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn tally_result(
        &self,
        proposal_id: u64,
    ) -> Result<QueryTallyResultResponse, CosmosClient> {
        let query = QueryTallyResultRequest { proposal_id };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmos.gov.v1beta1.Query/TallyResult".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryTallyResultResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
