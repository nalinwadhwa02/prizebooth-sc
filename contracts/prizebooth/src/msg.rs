use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;
use cw721::Cw721ReceiveMsg;
use crate::state::Pool;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub cw20_addr: String,
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
    RemoveNft { poolid: u32, token_id: String, nft_contract: String },

    Receive (Cw20ReceiveMsg),
    UpdateAdmin { addr: String },
}

#[cw_serde]
pub struct PoolInfoResponse {
    pub pool: Pool,
}

#[cw_serde]
pub struct NumPoolsResponse {
    pub pools: u32,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PoolInfoResponse)]
    PoolInfo {poolid: u32},
    #[returns(NumPoolsResponse)]
    NumPools {},
}