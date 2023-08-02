use crate::error::CosmosClient;
use crate::error::CosmosClient::{ProstDecodeError, RpcError};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmwasm::wasm::v1::{
    QueryAllContractStateRequest, QueryAllContractStateResponse, QueryCodeRequest,
    QueryCodeResponse, QueryCodesRequest, QueryCodesResponse, QueryContractHistoryRequest,
    QueryContractHistoryResponse, QueryContractInfoRequest, QueryContractInfoResponse,
    QueryContractsByCodeRequest, QueryContractsByCodeResponse, QueryPinnedCodesRequest,
    QueryPinnedCodesResponse, QueryRawContractStateRequest, QueryRawContractStateResponse,
    QuerySmartContractStateRequest,
};
use tendermint::abci::Code;
use prost::Message;
use serde::de::DeserializeOwned;
use serde::Serialize;
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
    pub async fn contract_info(
        &self,
        address: &str,
    ) -> Result<QueryContractInfoResponse, CosmosClient> {
        let query = QueryContractInfoRequest {
            address: address.to_string(),
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/ContractInfo".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryContractInfoResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn contract_history(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryContractHistoryResponse, CosmosClient> {
        let query = QueryContractHistoryRequest {
            address: address.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/ContractHistory".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryContractHistoryResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn contracts_by_code(
        &self,
        code_id: u64,
        pagination: Option<PageRequest>,
    ) -> Result<QueryContractsByCodeResponse, CosmosClient> {
        let query = QueryContractsByCodeRequest {
            code_id,
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/ContractsByCode".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryContractsByCodeResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn all_contract_state(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAllContractStateResponse, CosmosClient> {
        let query = QueryAllContractStateRequest {
            address: address.to_string(),
            pagination,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/AllContractState".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryAllContractStateResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn raw_contract_state(
        &self,
        address: &str,
        query_data: Vec<u8>,
    ) -> Result<QueryRawContractStateResponse, CosmosClient> {
        let query = QueryRawContractStateRequest {
            address: address.to_string(),
            query_data,
        };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/RawContractState".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryRawContractStateResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    /// - a Json Serialize Deserialize error
    pub async fn smart_contract_state<T: Serialize + Clone, U: DeserializeOwned>(
        &self,
        address: &str,
        msg: T,
    ) -> Result<U, CosmosClient> {
        let query = QuerySmartContractStateRequest {
            address: address.to_string(),
            query_data: serde_json::to_vec(&msg)?,
        };
        let ret = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/SmartContractState".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if ret.code != Code::Ok {
            return Err(RpcError(ret.log));
        }
        let resp: QueryRawContractStateResponse =
            QueryRawContractStateResponse::decode(ret.value.as_slice())?;

        serde_json::from_slice::<U>(resp.data.as_slice()).map_err(CosmosClient::JsonError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn code(&self, code_id: u64) -> Result<QueryCodeResponse, CosmosClient> {
        let query = QueryCodeRequest { code_id };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/Code".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryCodeResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn codes(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryCodesResponse, CosmosClient> {
        let query = QueryCodesRequest { pagination };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/Codes".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryCodesResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    /// # Errors
    ///
    /// Will return `Err` if :
    /// - a prost encode / decode fail
    /// - the json-rpc return an error code
    /// - if there is some network error
    pub async fn pinned_codes(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryPinnedCodesResponse, CosmosClient> {
        let query = QueryPinnedCodesRequest { pagination };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/PinnedCodes".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await?;

        if query.code != Code::Ok {
            return Err(RpcError(query.log));
        }
        QueryPinnedCodesResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
