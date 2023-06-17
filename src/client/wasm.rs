use crate::error::CosmosClientError;
use crate::error::CosmosClientError::ProstDecodeError;
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmwasm::wasm::v1::{
    QueryAllContractStateRequest, QueryAllContractStateResponse, QueryCodeRequest,
    QueryCodeResponse, QueryCodesRequest, QueryCodesResponse, QueryContractHistoryRequest,
    QueryContractHistoryResponse, QueryContractInfoRequest, QueryContractInfoResponse,
    QueryContractsByCodeRequest, QueryContractsByCodeResponse, QueryPinnedCodesRequest,
    QueryPinnedCodesResponse, QueryRawContractStateRequest, QueryRawContractStateResponse,
    QuerySmartContractStateRequest,
};
use prost::Message;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

pub struct WasmModule {
    rpc: Rc<HttpClient>,
}

impl WasmModule {
    pub fn new(rpc: Rc<HttpClient>) -> Self {
        WasmModule { rpc }
    }

    pub async fn contract_info(
        &self,
        address: &str,
    ) -> Result<QueryContractInfoResponse, CosmosClientError> {
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

        QueryContractInfoResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn contract_history(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryContractHistoryResponse, CosmosClientError> {
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

        QueryContractHistoryResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn contracts_by_code(
        &self,
        code_id: u64,
        pagination: Option<PageRequest>,
    ) -> Result<QueryContractsByCodeResponse, CosmosClientError> {
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

        QueryContractsByCodeResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn all_contract_state(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAllContractStateResponse, CosmosClientError> {
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

        QueryAllContractStateResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn raw_contract_state(
        &self,
        address: &str,
        query_data: Vec<u8>,
    ) -> Result<QueryRawContractStateResponse, CosmosClientError> {
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

        QueryRawContractStateResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn smart_contract_state<T: Serialize + Clone, U: DeserializeOwned>(
        &self,
        address: &str,
        msg: T,
    ) -> Result<U, CosmosClientError> {
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

        let resp: QueryRawContractStateResponse =
            QueryRawContractStateResponse::decode(ret.value.as_slice())?;

        serde_json::from_slice::<U>(resp.data.as_slice()).map_err(CosmosClientError::JsonError)
    }

    pub async fn code(&self, code_id: u64) -> Result<QueryCodeResponse, CosmosClientError> {
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

        QueryCodeResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn codes(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryCodesResponse, CosmosClientError> {
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

        QueryCodesResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }

    pub async fn pinned_codes(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryPinnedCodesResponse, CosmosClientError> {
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

        QueryPinnedCodesResponse::decode(query.value.as_slice()).map_err(ProstDecodeError)
    }
}
