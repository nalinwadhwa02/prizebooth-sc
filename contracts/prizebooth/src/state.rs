use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::{Map, Item};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[cw_serde]
pub enum PoolState {
    Open,
    Closed,
}

impl std::fmt::Display for PoolState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PoolState::Open => write!(f, "Open"),
            PoolState::Closed => write!(f, "Closed"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Prizepool {
    pub admin: Addr,
    pub creation_time: Timestamp,
    pub nft_list: Vec<String>,
    pub mintprice: Uint128,
    pub state: PoolState,
}

pub const POOLS: Map<u32, Prizepool> = Map::new("prizepools");
pub const POOLSIZE: Item<u32> = Item::new("prizepoolslen");

pub const CW721_ADDR:Item<Addr> = Item::new("nftcontract_addr");
pub const CW20_ADDR:Item<Addr> = Item::new("tokencontract_addr");