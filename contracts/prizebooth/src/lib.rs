use cosmwasm_std::{entry_point, MessageInfo, Deps, DepsMut, Response, StdResult, Env, Binary };

pub mod msg;
pub mod error;
pub mod test;
pub mod execute;
pub mod query; 
pub mod state;

use msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use error::ContractError;
use state::{CW20_ADDR, CW721_ADDR, NUMPOOLS, ADMIN};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate (
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    CW20_ADDR.save(deps.storage, &deps.api.addr_validate(msg.cw20_addr.as_str())?)?;
    CW721_ADDR.save(deps.storage, &deps.api.addr_validate(msg.cw721_addr.as_str())?)?;
    NUMPOOLS.save(deps.storage, &0)?;
    ADMIN.save(deps.storage, &deps.api.addr_validate(msg.admin.as_str())?)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute (
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePool { price } => execute::create_pool (deps, env, info, price),
        ExecuteMsg::RemovePool { poolid } => execute::remove_pool (deps, env, info, poolid),
        ExecuteMsg::ReceiveNft (msg) => execute::recieve_nft (deps, env, info, msg),
        ExecuteMsg::RemoveNft { poolid, token_id } => execute::remove_nft (deps, env, info, poolid, token_id),
        ExecuteMsg::Receive (msg) => execute::recieve_token (deps, env, info, msg),
        ExecuteMsg::UpdateAdmin { addr } => execute::update_admin (deps, env, info, addr),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query (
    _deps: Deps,
    _env: Env,
    _msg: QueryMsg,
) -> StdResult<Binary> {
    Ok(Binary::from(b""))
}