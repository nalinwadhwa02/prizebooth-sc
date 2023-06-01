use cosmwasm_std::{entry_point, MessageInfo, Deps, DepsMut, Response, StdResult, Env, Binary };

use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use contract::{execute, instantiate, query};
use error::ContractError;

pub mod msg;
pub mod contract;
pub mod error;
pub mod state;
pub mod tests;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate (
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    instantiate::instantiate(deps, env, info, msg.cw20_addr, msg.cw721_addr)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute (
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePrizePool{ mintprice } => execute::create_prize_pool(deps, env, info, mintprice),
        ExecuteMsg::Receive(msg) => execute::recieve_token(deps, env, info, msg),
        ExecuteMsg::ReceiveNft(msg) => execute::recieve_nft(deps, env, info, msg),
        ExecuteMsg::RemoveNft { poolid, token_id } => execute::remove_nft(deps, env, info, poolid, token_id),
        ExecuteMsg::RemovePrizePool { poolid } => execute::remove_prizepool(deps, env, info, poolid),
        ExecuteMsg::ChangePoolState { poolid, state } => execute::change_pool_state(deps, env, info, poolid, state),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query (
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::PoolList {  } => query::get_pool_list(deps, env)
    }
}