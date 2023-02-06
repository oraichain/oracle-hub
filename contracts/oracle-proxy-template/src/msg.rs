use cosmwasm_schema::{cw_serde, QueryResponses};
use tefi_oracle::proxy::{ProxyPriceResponse, ProxyQueryMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub source_addr: String,
}

#[cw_serde]
pub struct ConfigResponse {
    pub source_addr: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ProxyPriceResponse)]
    Base(ProxyQueryMsg),
    #[returns(ConfigResponse)]
    Config {},
}
