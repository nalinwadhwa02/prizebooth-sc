use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;
use cw721::Cw721ReceiveMsg;
#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub cw20_addr: String,
    pub cw721_addr: String,
}

#[cw_serde]
pub enum RecieveTokenMsg {
    RedeemToken {poolid: u32},
}

#[cw_serde]
pub enum RecieveNftMsg {
    AddNft { poolid: u32},
}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePool { price: Uint128 },
    RemovePool { poolid: u32 },

    ReceiveNft (Cw721ReceiveMsg),
    RemoveNft { poolid: u32, token_id: String },

    Receive (Cw20ReceiveMsg),
    UpdateAdmin { addr: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
}