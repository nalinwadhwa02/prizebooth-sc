use cosmwasm_std::{Addr, StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{sender} is not contract admin")]
    Unauthorized { sender: Addr },
    #[error("{poolid} pool not empty")]
    NonEmptyPool {poolid: u32},
    #[error("CW20 token amount {amt} not equal to redeem price {price}")]
    TokenAmountMismatch {amt: Uint128, price: Uint128},
    #[error("Faulty token contract: {addr}")]
    FaultyTokenContract {addr: String},
    #[error("Faulty nft contract: {addr}")]
    FaultyNftContract {addr: String},
    #[error("Pool {poolid} Empty")]
    EmptyPool {poolid: u32},
}