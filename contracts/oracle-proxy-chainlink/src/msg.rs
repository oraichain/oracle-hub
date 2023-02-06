use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
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
    /// Registers new sources, overwrites if already exists
    SetSources {
        sources: Vec<(String, String)>, // (symbol, source)
    },
    /// Removes an existing source
    RemoveSource {
        symbol: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ProxyPriceResponse)]
    Base(ProxyQueryMsg),
    #[returns(ConfigResponse)]
    Config {},
    #[returns(SourcesResponse)]
    Sources { symbol: Option<String> },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: String,
}

#[cw_serde]
pub struct SourcesResponse {
    pub sources: Vec<(String, String)>,
}

// Chainlink interfaces

#[cw_serde]
pub enum AggregatorQueryMsg {
    /// Query data for a specific round
    /// Response: [`RoundDataResponse`].
    GetRoundData {
        /// The round ID to retrieve the round data for
        round_id: u32,
    },
    /// Query data for the latest round
    /// Response: [`RoundDataResponse`].
    GetLatestRoundData {},

    GetDecimals {},

    GetDescription {},

    GetVersion {},

    GetLatestAnswer {},
}

#[cw_serde]
pub struct AggregatorQuery {
    pub aggregator_query: AggregatorQueryMsg,
}

#[cw_serde]
pub struct RoundDataResponse {
    pub round_id: u32,           // uint80
    pub answer: Option<Uint128>, // int256
    pub started_at: Option<u64>, // int256
    pub updated_at: Option<u64>, // uint256
    pub answered_in_round: u32,  // uint80
}
