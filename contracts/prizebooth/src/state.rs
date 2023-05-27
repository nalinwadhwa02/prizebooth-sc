use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Map, Item};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Prizepool {
    pub admin: Addr,
    pub creation_time: Timestamp,
    pub nft_list: Vec<String>,
    pub mintprice: Uint128,
}

pub const POOLS: Map<u32, Prizepool> = Map::new("prizepools");
pub const POOLSIZE: Item<u32> = Item::new("prizepoolslen");

pub const CW721_ADDR:Item<Addr> = Item::new("nftcontract_addr");
pub const CW20_ADDR:Item<Addr> = Item::new("tokencontract_addr");