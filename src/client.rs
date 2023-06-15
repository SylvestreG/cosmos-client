pub mod any_helper;
pub mod auth;
pub mod authz;
pub mod bank;
pub mod distribution;
pub mod evidence;
pub mod feegrant;
pub mod gov;
pub mod mint;
pub mod params;
pub mod slashing;
pub mod staking;
pub mod tx;
pub mod upgrade;
pub mod wasm;

use std::rc::Rc;
use tendermint_rpc::{Client, HttpClient};

use crate::client::auth::AuthModule;
use crate::client::authz::AuthzModule;
use crate::client::bank::BankModule;
use crate::client::distribution::DistributionModule;
use crate::client::evidence::EvidenceModule;
use crate::client::feegrant::FeeGrantModule;
use crate::client::gov::GovModule;
use crate::client::mint::MintModule;
use crate::client::params::ParamsModule;
use crate::client::slashing::SlashingModule;
use crate::client::staking::StakingModule;
use crate::client::tx::TxModule;
use crate::client::upgrade::UpgradeModule;
use crate::client::wasm::WasmModule;
use crate::error::CosmosClientError;

pub struct RpcClient {
    rpc: Rc<HttpClient>,
    chain_id: Option<String>,
    pub bank: BankModule,
    pub auth: AuthModule,
    pub authz: AuthzModule,
    pub distribution: DistributionModule,
    pub evidence: EvidenceModule,
    pub feegrant: FeeGrantModule,
    pub gov: GovModule,
    pub mint: MintModule,
    pub params: ParamsModule,
    pub slashing: SlashingModule,
    pub staking: StakingModule,
    pub tx: TxModule,
    pub upgrade: UpgradeModule,
    pub wasm: WasmModule,
}

impl RpcClient {
    pub fn new(url: &str) -> Result<Self, CosmosClientError> {
        let rpc = Rc::new(HttpClient::new(url)?);

        Ok(RpcClient {
            rpc: rpc.clone(),
            chain_id: None,
            auth: AuthModule::new(rpc.clone()),
            authz: AuthzModule::new(rpc.clone()),
            bank: BankModule::new(rpc.clone()),
            distribution: DistributionModule::new(rpc.clone()),
            evidence: EvidenceModule::new(rpc.clone()),
            feegrant: FeeGrantModule::new(rpc.clone()),
            gov: GovModule::new(rpc.clone()),
            mint: MintModule::new(rpc.clone()),
            params: ParamsModule::new(rpc.clone()),
            slashing: SlashingModule::new(rpc.clone()),
            staking: StakingModule::new(rpc.clone()),
            tx: TxModule::new(rpc.clone()),
            upgrade: UpgradeModule::new(rpc.clone()),
            wasm: WasmModule::new(rpc),
        })
    }

    pub async fn chain_id(&mut self) -> Result<String, CosmosClientError> {
        if self.chain_id.is_none() {
            self.chain_id = Some(self.rpc.status().await?.node_info.network.to_string())
        }

        Ok(self.chain_id.clone().unwrap())
    }
}
