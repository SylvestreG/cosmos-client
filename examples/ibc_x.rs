use colored::Colorize;
use cosmos_client::client::Rpc;
use cosmos_client::error::CosmosClient;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128};
use std::ops::Div;

#[cw_serde]
pub enum RangeOrder {
    Asc,
    Desc,
}

#[cw_serde]
pub struct SwapRoute {
    pub pool_id: u64,
    pub token_denom: String,
}

#[cw_serde]
pub struct SwapRoutes(pub Vec<SwapRoute>);

#[cw_serde]
pub struct SimulateMintResponse {
    pub mint_amount: Uint128,
    pub refund_amount: Vec<cosmwasm_std::Coin>,
    pub fund_spent: Vec<cosmwasm_std::Coin>,
}

#[cw_serde]
pub struct SimulateBurnResponse {
    pub burn_amount: Uint128,
    pub redeem_amount: Vec<cosmwasm_std::Coin>,
}

#[cw_serde]
pub struct TradeInfoPayload {
    pub denom_in: String,
    pub denom_out: String,
    pub routes: SwapRoutes,
    pub cooldown: u64,
    pub max_trade_amount: Uint128,
    pub last_traded_at: Option<u64>,
}

#[cw_serde]
pub struct ListTradeInfoResponse(pub Vec<TradeInfoPayload>);

#[cw_serde]
pub struct GetTradeInfoResponse {
    pub trade_info: Option<TradeInfoPayload>,
}

#[cw_serde]
pub struct RebalancePayload {
    pub manager: Option<Addr>,
    pub deflation: Vec<(String, Decimal)>,
    pub inflation: Vec<(String, Decimal)>,
}

#[cw_serde]
pub struct GetRebalanceResponse {
    pub rebalance: Option<RebalancePayload>,
}

#[cw_serde]
pub struct StreamingFeeResponse {
    pub rate: Decimal,
    pub collected: Vec<cosmwasm_std::Coin>,
    pub freeze: bool,
    pub last_collected_at: u64,
}

#[cw_serde]
pub struct GetConfigResponse {
    pub gov: Addr,
    pub pending_gov: Option<Addr>,
    pub paused: PausedResponse,
    pub index_denom: String,
    pub reserve_denom: String,
}

#[cw_serde]
pub struct GetFeeResponse {
    pub collector: Addr,
    pub mint_fee: Option<Decimal>,
    pub burn_fee: Option<Decimal>,
    pub streaming_fee: Option<StreamingFeeResponse>,
}

#[cw_serde]
pub struct GetPortfolioResponse {
    pub total_supply: Uint128,
    pub assets: Vec<cosmwasm_std::Coin>,
    pub units: Vec<(String, Decimal)>,
}

#[cw_serde]
pub struct PausedResponse {
    pub paused: bool,
    pub expires_at: Option<u64>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint128)]
    GetBalance { account: String },

    #[returns(Uint128)]
    GetTotalSupply {},

    #[returns(GetConfigResponse)]
    GetConfig { time: Option<u64> },

    #[returns(GetFeeResponse)]
    GetFee { time: Option<u64> },

    #[returns(GetPortfolioResponse)]
    GetPortfolio { time: Option<u64> },

    #[returns(GetRebalanceResponse)]
    GetRebalance {},

    #[returns(GetTradeInfoResponse)]
    GetTradeInfo { denom_in: String, denom_out: String },

    #[returns(ListTradeInfoResponse)]
    ListTradeInfo {
        denom_in: String,
        start_after: Option<String>,
        limit: Option<u32>,
        order: Option<RangeOrder>,
    },

    #[returns(SimulateMintResponse)]
    SimulateMint {
        amount: Uint128,
        funds: Option<Vec<cosmwasm_std::Coin>>,
        time: Option<u64>,
    },

    #[returns(SimulateBurnResponse)]
    SimulateBurn { amount: Uint128, time: Option<u64> },
}

struct IbcXQueries<'a> {
    rpc: &'a Rpc,
    contract: &'static str,
}

impl<'a> IbcXQueries<'a> {
    pub fn new(rpc: &'a Rpc, contract: &'static str) -> Self {
        IbcXQueries { rpc, contract }
    }

    #[allow(dead_code)]
    pub async fn get_balance(&self, account: String) -> Result<Uint128, CosmosClient> {
        self.rpc
            .wasm
            .smart_contract_state(self.contract, QueryMsg::GetBalance { account })
            .await
    }

    pub async fn get_total_supply(&self) -> Result<Uint128, CosmosClient> {
        self.rpc
            .wasm
            .smart_contract_state(self.contract, QueryMsg::GetTotalSupply {})
            .await
    }

    pub async fn get_config(&self, time: Option<u64>) -> Result<GetConfigResponse, CosmosClient> {
        self.rpc
            .wasm
            .smart_contract_state(self.contract, QueryMsg::GetConfig { time })
            .await
    }

    pub async fn get_fee(&self, time: Option<u64>) -> Result<GetFeeResponse, CosmosClient> {
        self.rpc
            .wasm
            .smart_contract_state(self.contract, QueryMsg::GetFee { time })
            .await
    }

    pub async fn get_portfolio(
        &self,
        time: Option<u64>,
    ) -> Result<GetPortfolioResponse, CosmosClient> {
        self.rpc
            .wasm
            .smart_contract_state(self.contract, QueryMsg::GetPortfolio { time })
            .await
    }

    #[allow(dead_code)]
    pub async fn get_rebalance(&self) -> Result<GetRebalanceResponse, CosmosClient> {
        self.rpc
            .wasm
            .smart_contract_state(self.contract, QueryMsg::GetRebalance {})
            .await
    }

    #[allow(dead_code)]
    pub async fn get_trade_info(
        &self,
        denom_in: String,
        denom_out: String,
    ) -> Result<GetTradeInfoResponse, CosmosClient> {
        self.rpc
            .wasm
            .smart_contract_state(
                self.contract,
                QueryMsg::GetTradeInfo {
                    denom_in,
                    denom_out,
                },
            )
            .await
    }

    #[allow(dead_code)]
    pub async fn list_trade_info(
        &self,
        denom_in: String,
    ) -> Result<ListTradeInfoResponse, CosmosClient> {
        self.rpc
            .wasm
            .smart_contract_state(
                self.contract,
                QueryMsg::ListTradeInfo {
                    denom_in,
                    order: None,
                    start_after: None,
                    limit: Some(30),
                },
            )
            .await
    }
}

#[tokio::main]
async fn main() -> Result<(), CosmosClient> {
    env_logger::init();

    let rpc = Rpc::new("https://osmosis-rpc.polkachu.com").await?;
    let ibcx_contract = "osmo14klwqgkmackvx2tqa0trtg69dmy0nrg4ntq4gjgw2za4734r5seqjqm4gm";
    let ibcx = IbcXQueries::new(&rpc, ibcx_contract);
    let ibcx_config = ibcx.get_config(None).await?;
    let fee = ibcx.get_fee(None).await?;
    let portfolio = ibcx.get_portfolio(None).await?;

    println!(
        "totalSupply {} IBCX",
        ibcx.get_total_supply()
            .await?
            .div(Uint128::from(1_000_000u128))
            .to_string()
            .blue()
    );
    println!("gov addr {}", ibcx_config.gov.to_string().red());
    println!("ibx denom {}", ibcx_config.index_denom.green());
    println!("idx denom1 {}", ibcx_config.reserve_denom.green());
    println!("fee collector {}", fee.collector.to_string().red());
    println!(
        "mint fee {}",
        fee.mint_fee.unwrap_or_default().to_string().blue()
    );
    println!(
        "burn fee {}",
        fee.burn_fee.unwrap_or_default().to_string().blue()
    );
    println!(
        "portfolio totalSupply {} IBCX",
        portfolio
            .total_supply
            .div(Uint128::from(1_000_000u128))
            .to_string()
            .blue()
    );
    for asset in portfolio.assets {
        if asset.denom == "ibc/6AE98883D4D5D5FF9E50D7130F1305DA2FFA0C652D1DD9C123657C6B4EB2DF8A" {
            println!(
                "\t - asset {} {}",
                asset
                    .amount
                    .div(Uint128::from(1_000_000_000_000_000_000u128))
                    .to_string()
                    .blue(),
                asset.denom.to_string().green()
            );
        } else {
            println!(
                "\t - asset {} {}",
                asset
                    .amount
                    .div(Uint128::from(1_000_000u128))
                    .to_string()
                    .blue(),
                asset.denom.to_string().green()
            );
        }
    }

    Ok(())
}
