use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmos_sdk_proto::cosmwasm::wasm::v1::{
    QueryAllContractStateRequest, QueryAllContractStateResponse, QueryCodeRequest,
    QueryCodeResponse, QueryCodesRequest, QueryCodesResponse, QueryContractHistoryRequest,
    QueryContractHistoryResponse, QueryContractInfoRequest, QueryContractInfoResponse,
    QueryContractsByCodeRequest, QueryContractsByCodeResponse, QueryPinnedCodesRequest,
    QueryPinnedCodesResponse, QueryRawContractStateRequest, QueryRawContractStateResponse,
    QuerySmartContractStateRequest, QuerySmartContractStateResponse,
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
    ) -> Result<QueryContractInfoResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryContractInfoResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn contract_history(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryContractHistoryResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryContractHistoryResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn contracts_by_code(
        &self,
        code_id: u64,
        pagination: Option<PageRequest>,
    ) -> Result<QueryContractsByCodeResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryContractsByCodeResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn all_contract_state(
        &self,
        address: &str,
        pagination: Option<PageRequest>,
    ) -> Result<QueryAllContractStateResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryAllContractStateResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn raw_contract_state(
        &self,
        address: &str,
        query_data: Vec<u8>,
    ) -> Result<QueryRawContractStateResponse, anyhow::Error> {
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
            .await
            .unwrap();

        let resp = QueryRawContractStateResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn smart_contract_state<T: Serialize + Clone, U: DeserializeOwned>(
        &self,
        address: &str,
        msg: T,
    ) -> Result<U, anyhow::Error> {
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
            .await
            .unwrap();

        let resp: QueryRawContractStateResponse =
            QueryRawContractStateResponse::decode(ret.value.as_slice())?;
        println!("{:#?}", ret);
        Ok(serde_json::from_slice::<U>(&resp.data.as_slice())?)
    }

    pub async fn code(&self, code_id: u64) -> Result<QueryCodeResponse, anyhow::Error> {
        let query = QueryCodeRequest { code_id };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/Code".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryCodeResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn codes(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryCodesResponse, anyhow::Error> {
        let query = QueryCodesRequest { pagination };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/Codes".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryCodesResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }

    pub async fn pinned_codes(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<QueryPinnedCodesResponse, anyhow::Error> {
        let query = QueryPinnedCodesRequest { pagination };
        let query = self
            .rpc
            .abci_query(
                Some("/cosmwasm.wasm.v1.Query/PinnedCodes".to_string()),
                query.encode_to_vec(),
                None,
                false,
            )
            .await
            .unwrap();

        let resp = QueryPinnedCodesResponse::decode(query.value.as_slice())?;
        Ok(resp)
    }
}
