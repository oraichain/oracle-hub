use cosmwasm_schema::write_api;

use tefi_oracle::hub::{
    HubExecuteMsg as ExecuteMsg, HubQueryMsg as QueryMsg, InstantiateMsg, MigrateMsg,
};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
        migrate: MigrateMsg,
    }
}
