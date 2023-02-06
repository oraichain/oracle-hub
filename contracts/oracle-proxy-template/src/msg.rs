use cosmwasm_schema::cw_serde;
use tefi_oracle::proxy::ProxyQueryMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub source_addr: String,
}

#[cw_serde]
pub struct ConfigResponse {
    pub source_addr: String,
}

#[cw_serde]
pub enum QueryMsg {
    Base(ProxyQueryMsg),
    Config {},
}
