use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub enum ProxyQueryMsg {
    Price { symbol: String },
}

#[cw_serde]
pub struct ProxyPriceResponse {
    pub rate: Decimal,     // rate denominated in base_denom
    pub last_updated: u64, // timestamp in seconds
}

#[cw_serde]
pub enum ProxyBaseQuery {
    Base(ProxyQueryMsg),
}
