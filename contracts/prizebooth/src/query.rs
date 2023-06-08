use cosmwasm_std::{StdResult, Binary, Deps, Env, to_binary};
use crate::{state::{POOLS, NUMPOOLS}, msg::{PoolInfoResponse, NumPoolsResponse}};

pub fn query_pool_info (
    deps: Deps,
    _env: Env,
    poolid: u32,
) -> StdResult<Binary> {
    let pool = POOLS.may_load(deps.storage, poolid)?.unwrap();
    let resp = PoolInfoResponse {
        pool: pool
    };
    to_binary(&resp)
}

pub fn query_num_pools (
    deps: Deps,
    _env: Env,
) -> StdResult<Binary> {
    let numpools = NUMPOOLS.load(deps.storage)?;
    let resp = NumPoolsResponse {
        pools: numpools,
    };
    to_binary(&resp)
}