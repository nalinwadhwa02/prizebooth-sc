use cosmwasm_std::{MessageInfo, Response, DepsMut, Env, from_binary, to_binary, StdResult, Deps, Binary, Order, Addr, Uint128 };
use cw20::{Cw20ReceiveMsg, Balance, Cw20CoinVerified};
use cw721::{Cw721ReceiveMsg};
use rand::{thread_rng, Rng};
use my_cw721;
use my_cw20;

use crate::state::{POOLS, Prizepool, POOLSIZE, CW20_ADDR, CW721_ADDR, PoolState};
use crate::error::ContractError;

use crate::msg::{ReceiveTokenMsg, ReceiveNftMsg };

pub mod instantiate {

    use super::*;

    pub fn instantiate (
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        cw20_addr: String,
        cw721_addr: String
    ) -> StdResult<Response> {
        let cw20 = deps.api.addr_validate(&cw20_addr)?;
        let cw721 = deps.api.addr_validate(&cw721_addr)?;
        CW721_ADDR.save(deps.storage, &cw721)?;
        CW20_ADDR.save(deps.storage, &cw20)?;
        let resp = Response::new()
            .add_attribute("action", "instantiate")
            .add_attribute("instantiate_address", info.sender.to_string())
            .add_attribute("cw721_address", cw721_addr)
            .add_attribute("cw20_address", cw20_addr);
        Ok(resp)
    }
}

pub mod execute {

    use super::*;

    pub fn create_prize_pool (
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        mp: Uint128,
    ) -> Result<Response, ContractError> {
        let newpool = Prizepool{
            admin: info.sender.clone(),
            creation_time: env.block.time.clone(),
            nft_list: vec![],
            mintprice: mp,
            state: PoolState::Closed,
        };
        let mut poolsize = POOLSIZE.may_load(deps.storage)?.unwrap_or(0);
        POOLS.save(deps.storage, poolsize.clone(), &newpool)?;
        poolsize += 1;
        POOLSIZE.save(deps.storage, &poolsize)?;
        

        let resp = Response::new()
            .add_attribute("action", "create_pool")
            .add_attribute("creator", info.sender.to_string())
            .add_attribute("poolid", (poolsize-1).to_string())
            .add_attribute("mintprice", mp.to_string());
        Ok(resp)

    }

    pub fn recieve_token(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw20ReceiveMsg
    ) -> Result<Response, ContractError> {
        //confirm recieved token from valid contract
        let tokentract = CW20_ADDR.load(deps.storage)?;
        if tokentract != info.sender.clone() {
            return Err(ContractError::UnauthorizedTokenContract { contract: info.sender.clone()});
        }

        let humanminter = deps.api.addr_validate(&msg.sender)?;
        let rmsg: ReceiveTokenMsg = from_binary(&msg.msg)?;
        let balance = Balance::Cw20(Cw20CoinVerified {
            address: humanminter.clone(),
            amount: msg.amount,
        });

        match rmsg {
            ReceiveTokenMsg::Mint { poolid } => Ok(mint(deps, env, info, poolid, humanminter, msg.amount)
                .unwrap()
                .add_attribute("balance", balance.to_string()))
        }
    
    }

    pub fn recieve_nft(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721ReceiveMsg
    ) -> Result<Response, ContractError> {
        //confirm recieved nft from valid contract
        let nfttract = CW721_ADDR.load(deps.storage)?;
        if nfttract != info.sender.clone() {
            return Err(ContractError::UnauthorizedNFTContract { contract: info.sender.clone() });
        } 

        let rmsg : ReceiveNftMsg = from_binary(&msg.msg)?;
        match rmsg {
            ReceiveNftMsg::AddNft { poolid } => addnft(deps, env, info, poolid, msg.sender.clone(), msg.token_id.clone())
        }
    }

    fn mint(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        poolid: u32,
        minter: Addr,
        tokensent: Uint128,
    ) -> Result<Response, ContractError> { 

        let mut pool = POOLS.may_load(deps.storage, poolid)?.unwrap();
        if pool.state == PoolState::Closed {
            return Err(ContractError::PoolStateClosed { poolid: poolid });
        }

        let nftlistsize = pool.nft_list.len();
        //handle empty list edge case
        if nftlistsize == 0 {
            return Err(ContractError::ZeroLenNftList { poolid: poolid });
        }
        //check if balance is enough to buy
        if pool.mintprice != tokensent {
            return Err(ContractError::UnequalTokensToMint { t: tokensent });
        }

        //select nft at random
        let mut rng = thread_rng();
        let nftindex = rng.gen_range(0..nftlistsize);
        let nft = pool.nft_list[nftindex].clone();
        pool.nft_list.remove(nftindex);
        let update_prizepool = |d: Option<Prizepool> | -> StdResult<Prizepool> {
            match d {
                Some(_) => Ok(pool.clone()),
                None => Ok(pool.clone()),
            }
        };
        POOLS.update(deps.storage, poolid, update_prizepool)?;

        //msg for nfttract
        let nfttract_addr = CW721_ADDR.load(deps.storage)?;
        let msg = to_binary(&my_cw721::msg::PrizeBoothMsg::TransferNftPb { recpt: minter.to_string(), token_id: nft.clone() }).unwrap();
        let sendnft = my_cw721::msg::PBRM{
            sender: info.sender.clone().to_string(),
            msg: msg,
        };

        //msg for token transfer to admin
        let tokentract = CW20_ADDR.load(deps.storage)?;
        let msg = to_binary(&my_cw20::msg::PrizeBoothMsg::TransferTokens { recpt: pool.admin.to_string(), amount: tokensent }).unwrap();
        let sendtoken = my_cw20::msg::PBRM {
            sender: minter.to_string(), 
            msg: msg,
        };

        
        let resp = Response::new()
            .add_message(sendnft.into_cosmos_msg(nfttract_addr.to_string())?)
            .add_message(sendtoken.into_cosmos_msg(tokentract.to_string())?)
            .add_attribute("action", "mint")
            .add_attribute("minter", minter.to_string())
            .add_attribute("poolid", poolid.to_string())
            .add_attribute("nft_minted",  nft);
        Ok(resp)
    }

    fn addnft(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        poolid: u32,
        sender: String,
        token_id: String,
    ) -> Result<Response, ContractError> {
        let sender = deps.api.addr_validate(&sender)?;
        let mut prizepool = POOLS.may_load(deps.storage, poolid)?.unwrap();
        if  prizepool.admin != sender {
            return Err(ContractError::UnauthorizedAdmin { sender: prizepool.admin.clone()})
        }
        prizepool.nft_list.push(token_id.clone());
        let update_prizepool = |d: Option<Prizepool> | -> StdResult<Prizepool> {
            match d {
                Some(_) => Ok(prizepool),
                None => Ok(prizepool),
            }
        };
        POOLS.update(deps.storage, poolid, update_prizepool)?;

        let resp = Response::new()
            .add_attribute("action", "add_nft")
            .add_attribute("poolid", poolid.to_string())
            .add_attribute("sender", sender.to_string())
            .add_attribute("token_id", token_id.to_string())
            .add_attribute("nft_contract_address", info.sender.to_string());

        Ok(resp)
    }

    pub fn remove_nft(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        poolid: u32,
        token_id: String
    ) -> Result<Response, ContractError> {
        let mut pool = POOLS.may_load(deps.storage, poolid)?.unwrap();
        if pool.admin != info.sender.clone() {
            return Err(ContractError::UnauthorizedAdmin { sender: info.sender });
        }

        pool.nft_list.retain(|f| f != &token_id);
        let update_prizepool = |d: Option<Prizepool> | -> StdResult<Prizepool> {
            match d {
                Some(_) => Ok(pool),
                None => Ok(pool),
            }
        };
        POOLS.update(deps.storage, poolid, update_prizepool)?;
        //send nft back to the admin
        let nfttract_addr = CW721_ADDR.load(deps.storage)?;
        let msg = to_binary(&my_cw721::msg::PrizeBoothMsg::TransferNftPb { recpt: info.sender.clone().to_string(), token_id: token_id.clone() }).unwrap();
        let sendnft = my_cw721::msg::PBRM{
            sender: info.sender.clone().to_string(),
            msg: msg,
        };

        let resp = Response::new()
            .add_message(sendnft.into_cosmos_msg(nfttract_addr.to_string())?)
            .add_attribute("action", "remove_nft")
            .add_attribute("poolid", poolid.to_string())
            .add_attribute("tokenid", token_id)
            .add_attribute("admin", info.sender.to_string());
        Ok(resp)

    }

    pub fn remove_prizepool(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        poolid: u32
    ) -> Result<Response, ContractError> {
        let pool = POOLS.may_load(deps.storage, poolid)?.unwrap();
        if pool.admin != info.sender.clone() {
            return Err(ContractError::UnauthorizedAdmin { sender: info.sender });
        }
        if pool.nft_list.len() > 0 {
            return Err(ContractError::NonZeroNftList { poolid: poolid });
        }

        POOLS.remove(deps.storage, poolid);
        let resp = Response::new()
            .add_attribute("action", "remove_prizepool")
            .add_attribute("poolid", poolid.to_string())
            .add_attribute("admin", info.sender.to_string());
        Ok(resp)
    }

    pub fn change_pool_state (
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        poolid: u32,
        state: PoolState
    ) -> Result<Response, ContractError> {
        let mut pool = POOLS.may_load(deps.storage, poolid)?.unwrap();
        if pool.admin != info.sender.clone() {
            return Err(ContractError::UnauthorizedAdmin { sender: info.sender });
        }
        pool.state = state.clone();
        let update_prizepool = |d: Option<Prizepool> | -> StdResult<Prizepool> {
            match d {
                Some(_) => Ok(pool),
                None => Ok(pool),
            }
        };
        POOLS.update(deps.storage, poolid, update_prizepool)?;
        let resp = Response::new()
            .add_attribute("action", "change_pool_state")
            .add_attribute("admin", info.sender.to_string())
            .add_attribute("state", state.to_string())
            .add_attribute("poolid", poolid.to_string());

        Ok(resp)
    }



}

pub mod query {
    use super::*;


    pub fn get_pool_list (
        deps: Deps,
        _env: Env,
    ) -> StdResult<Binary> {
        let mut resp = vec![];
        for item in POOLS.range(deps.storage, None, None, Order::Ascending) {
            let (_, v) = item?;
            resp.push(v);
        }
        Ok(to_binary(&resp)?)
    }

}
