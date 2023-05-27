use thiserror::Error;
use cosmwasm_std::{StdError, Addr, Uint128};

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("{sender} is not pool admin")]
    UnauthorizedAdmin {sender: Addr},
    #[error("{contract} is not Authorized Token Contract")]
    UnauthorizedTokenContract {contract: Addr},
    #[error("{contract} is not Authorized NFT Contract")]
    UnauthorizedNFTContract {contract: Addr},
    #[error("{t} tokens not same as nft mint price")]
    UnequalTokensToMint {t: Uint128},
    #[error("{poolid} has no NFTs to mint")]
    ZeroLenNftList {poolid: u32},
}