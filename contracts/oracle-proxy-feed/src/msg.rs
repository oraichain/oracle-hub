use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;
use tefi_oracle::proxy::{ProxyPriceResponse, ProxyQueryMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateOwner {
        owner: String,
    },
    /// Used to register new asset or to update feeder
    RegisterFeed {
        symbol: String,
        feeder: String,
    },
    FeedPrices {
        prices: Vec<(String, Decimal)>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ProxyPriceResponse)]
    Base(ProxyQueryMsg),
    #[returns(ConfigResponse)]
    Config {},
    #[returns(FeederResponse)]
    Feeder { symbol: String },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: String,
}

#[cw_serde]
pub struct FeederResponse {
    pub symbol: String,
    pub feeder: String,
}
