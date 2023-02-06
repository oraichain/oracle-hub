use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::msg::ConfigResponse;

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct Config {
    pub source_addr: Addr,
}

impl Config {
    pub fn as_res(&self) -> ConfigResponse {
        ConfigResponse {
            source_addr: self.source_addr.to_string(),
        }
    }
}
