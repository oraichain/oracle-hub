use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use tefi_oracle::proxy::{ProxyPriceResponse, ProxyQueryMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub source_addr: String,
    pub quote_symbol: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<String>,
        source_addr: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ProxyPriceResponse)]
    Base(ProxyQueryMsg),
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: String,
    pub source_addr: String,
    pub quote_symbol: String,
}

/// Band Protocol interface

#[cw_serde]
pub struct BandResponse {
    pub rate: Uint128,
    pub last_updated_base: u64,
    pub last_updated_quote: u64,
}

#[cw_serde]
pub enum BandMsg {
    GetReferenceData {
        base_symbol: String,
        quote_symbol: String,
    },
}
