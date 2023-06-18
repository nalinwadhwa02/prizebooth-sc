use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr, Uint128};

pub const CW20_ADDR: Item<Addr> = Item::new("cw20_addr");
pub const ADMIN: Item<Addr> = Item::new("admin");

#[cw_serde]
pub struct Pool {
    pub tokens: Vec<(String, String)>,
    pub price: Uint128,
}

pub const POOLS: Map<u32, Pool> = Map::new("pools");
pub const NUMPOOLS: Item<u32> = Item::new("numpools");