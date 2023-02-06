use cosmwasm_schema::cw_serde;

use cosmwasm_std::Decimal;

pub const DEFAULT_PRIORITY: u8 = 10;
pub const MAX_WHITELISTED_PROXIES: u8 = 15;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub base_denom: String,
    pub max_proxies_per_symbol: u8,
}

#[cw_serde]
pub enum HubExecuteMsg {
    /// Owner operation to update the owner parameter
    UpdateOwner { owner: String },
    /// Owner operation to update the max_proxies_per_symbol parameter
    UpdateMaxProxies { max_proxies_per_symbol: u8 },
    /// Register a new source for a symbol
    RegisterSource {
        symbol: String,
        proxy_addr: String,
        priority: Option<u8>,
    },
    /// Registers a list of sources
    BulkRegisterSource {
        sources: Vec<(String, String, Option<u8>)>, // (symbol, proxy_addr, priority)
    },
    /// Updates the priorities for proxies registered
    UpdateSourcePriorityList {
        symbol: String,
        priority_list: Vec<(String, u8)>,
    },
    /// Removes an already registered proxy
    RemoveSource { symbol: String, proxy_addr: String },
    /// Whitelists a new proxy in hub. After a proxy is whitelisted
    /// it can be registered as a source
    WhitelistProxy {
        proxy_addr: String,
        provider_name: String,
    },
    /// Removes a proxy from the whitelist
    RemoveProxy { proxy_addr: String },
    /// Updates the map of `asset_token` to `symbol`
    /// overwrites storage if already mapped
    InsertAssetSymbolMap {
        map: Vec<(String, String)>, // (address, symbol)
    },
}

#[cw_serde]
pub enum HubQueryMsg {
    /// Queries contract configuration
    Config {},
    /// Queries the list of whitelisted proxies
    ProxyWhitelist {},
    /// Returns the list of all symbols with all the sources
    AllSources {
        start_after: Option<String>, // symbol for pagination
        limit: Option<u32>,
    },
    /// Queries the information of all registered proxies for the provided asset_token
    Sources { asset_token: String },
    /// Queries the information of all registered proxies for the provided symbol
    SourcesBySymbol { symbol: String },
    /// Queries the highes priority available price within the timeframe
    /// If timeframe is not provided, it will ignore the price age
    Price {
        asset_token: String,
        timeframe: Option<u64>,
    },
    /// Queries the highes priority available price within the timeframe
    /// If timeframe is not provided, it will ignore the price age
    PriceBySymbol {
        symbol: String,
        timeframe: Option<u64>,
    },
    /// Queries all registered proxy prices for the provied asset_token
    PriceList { asset_token: String },
    /// Queries all registered proxy prices for the provied symbol
    PriceListBySymbol { symbol: String },
    /// Returns the map of `asset_token` to `symbol`
    AssetSymbolMap {
        start_after: Option<String>, // address for pagination
        limit: Option<u32>,
    },
    /// Query to check if `proxy_addr` is whitelisted and has price feed
    /// for the specified `symbol`. The purpose of this query is to have a
    /// way of checking if a price feed is valid and available before registering
    /// Returns the PriceResponse or an error
    CheckSource { proxy_addr: String, symbol: String },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: String,
    pub base_denom: String,
    pub max_proxies_per_symbol: u8,
}

#[cw_serde]
pub struct PriceResponse {
    pub rate: Decimal,
    pub last_updated: u64,
}

#[cw_serde]
pub enum PriceQueryResult {
    Success(PriceResponse),
    Fail,
}

#[cw_serde]
pub struct PriceListResponse {
    pub price_list: Vec<(u8, ProxyInfoResponse, PriceQueryResult)>, // (priority, proxy_info, result)
}

#[cw_serde]
pub struct SourcesResponse {
    pub symbol: String,
    pub proxies: Vec<(u8, ProxyInfoResponse)>,
}

#[cw_serde]
pub struct AllSourcesResponse {
    pub list: Vec<SourcesResponse>,
}

impl From<crate::proxy::ProxyPriceResponse> for PriceResponse {
    fn from(proxy_res: crate::proxy::ProxyPriceResponse) -> Self {
        PriceResponse {
            rate: proxy_res.rate,
            last_updated: proxy_res.last_updated,
        }
    }
}

#[cw_serde]
pub struct ProxyWhitelistResponse {
    pub proxies: Vec<ProxyInfoResponse>,
}

#[cw_serde]
pub struct ProxyInfoResponse {
    pub address: String,
    pub provider_name: String,
}

#[cw_serde]
pub struct AssetSymbolMapResponse {
    pub map: Vec<(String, String)>, // address, symbol
}
