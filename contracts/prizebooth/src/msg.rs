use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128};
use cw20::Cw20ReceiveMsg;
use cw721::Cw721ReceiveMsg;

use crate::state::{Prizepool, PoolState};



#[cw_serde]
pub struct InstantiateMsg {
    pub cw721_addr: String,
    pub cw20_addr: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePrizePool { mintprice: Uint128 },
    RemovePrizePool {poolid: u32},
    Receive (Cw20ReceiveMsg),
    ReceiveNft (Cw721ReceiveMsg),
    RemoveNft {poolid: u32, token_id: String},
    ChangePoolState {poolid: u32, state: PoolState},
}

#[cw_serde]
pub enum ReceiveTokenMsg {
    Mint {poolid: u32},
}

#[cw_serde]
pub enum ReceiveNftMsg {
    AddNft {poolid: u32},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<Prizepool>)]
    PoolList {},
}