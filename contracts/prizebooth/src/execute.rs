use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, CosmosMsg, WasmMsg, to_binary, Uint128, from_binary, StdResult};
use cw20::{Cw20ReceiveMsg};
use cw721::Cw721ReceiveMsg;

use crate::msg::{RecieveNftMsg, RecieveTokenMsg};
use crate::{error::ContractError};
use crate::state::{NUMPOOLS, Pool, POOLS, ADMIN, CW20_ADDR, CW721_ADDR};

pub fn create_pool (
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    price: Uint128,
) -> Result<Response, ContractError> {
    let mut poolnum = NUMPOOLS.load(deps.storage)?;
    let newpool = Pool {
        tokens: vec![],
        price: price.clone(),
    };
    POOLS.save(deps.storage, poolnum.clone(), &newpool)?;
    poolnum += 1;
    NUMPOOLS.save(deps.storage, &poolnum)?;
    Ok(Response::new()
        .add_attribute("action", "create_pool")
        .add_attribute("poolid", (poolnum-1).to_string())
        .add_attribute("price", price.to_string())
        )
}

pub fn remove_pool (
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _poolid: u32,
) -> Result<Response, ContractError> {
    // let pool = POOLS.may_load(deps.storage, poolid)?.unwrap();
    // if pool.tokens.len() > 0 {
    //     //return contract_error nonzero token list
    // }
    Ok(Response::new())
}

pub fn recieve_nft (
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw721ReceiveMsg,
) -> Result<Response, ContractError> {
    let rmsg : RecieveNftMsg = from_binary(&msg.msg)?;
    let sender = deps.api.addr_validate(&msg.sender)?;
    if sender != ADMIN.load(deps.storage)? {
        //return error if someone other than admin accessed
    }
    match rmsg {
        RecieveNftMsg::AddNft { poolid} => add_nft (deps, env, info, poolid, msg.token_id),
    }
}

pub fn add_nft (
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    poolid: u32, 
    token_id: String,
) -> Result<Response, ContractError> {
    let mut pool = POOLS.may_load(deps.storage, poolid)?.unwrap();
    pool.tokens.push(token_id.clone());
    let update_pool= |d: Option<Pool>| -> StdResult<Pool> {
        match d {
            Some(_one) => Ok(pool),
            None => Ok(pool),
        }
    };   
    POOLS.update(deps.storage, poolid, update_pool)?;
    Ok(Response::new())
}

pub fn remove_nft (
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _poolid: u32,
    _token_id: String,
) -> Result<Response, ContractError> {
    //remove_nft
    Ok(Response::new())
}

pub fn recieve_token (
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let rmsg: RecieveTokenMsg = from_binary(&msg.msg)?;
    let amount: Uint128 = msg.amount;
    match rmsg {
        RecieveTokenMsg::RedeemToken { poolid } => redeem_token (deps, env, info, amount, poolid, msg.sender),
    }
}

pub fn redeem_token (
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    amount: Uint128,
    poolid: u32,
    sender: String,
) -> Result<Response, ContractError> {
    let mut pool = POOLS.may_load(deps.storage, poolid)?.unwrap();
    if amount < pool.clone().price {
        //return error amount not enough tokens
        //msg to send back tokens
    }
    //get nftindex to redeem
    let nftindex = 0;
    let token = pool.tokens[nftindex].clone();
    pool.tokens.remove(nftindex);
    let update_pool= |d: Option<Pool>| -> StdResult<Pool> {
        match d {
            Some(_one) => Ok(pool.clone()),
            None => Ok(pool.clone()),
        }
    };   
    POOLS.update(deps.storage, poolid, update_pool)?;
    //msg to burn tokens
    let cw20_msg = CosmosMsg::Wasm(WasmMsg::Execute { 
        contract_addr: CW20_ADDR.load(deps.storage)?.to_string(), 
        msg: to_binary(&cw20::Cw20ExecuteMsg::Burn { amount: pool.price })?, 
        funds: vec![],
    });
    //msg to send nft
    let cw721_msg = CosmosMsg::Wasm(WasmMsg::Execute { 
        contract_addr: CW721_ADDR.load(deps.storage)?.to_string(), 
        msg: to_binary(&cw721::Cw721ExecuteMsg::TransferNft { recipient: sender, token_id: token })?, 
        funds: vec![],
    });
    Ok(Response::new()
        .add_message(cw20_msg)
        .add_message(cw721_msg)
        )
}

pub fn update_admin (
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _newadmin: String,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}