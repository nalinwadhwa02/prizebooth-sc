
#[cfg(test)]
mod tests {
    use cw20::BalanceResponse;
    use cw721::TokensResponse;
    use cw_multi_test::{App, ContractWrapper, Contract, Executor};
    use cosmwasm_std::{Empty, Addr, Uint128, to_binary};

    use crate::{execute, instantiate, query, msg::{InstantiateMsg, ExecuteMsg, RecieveNftMsg, RecieveTokenMsg, NumPoolsResponse, QueryMsg, PoolInfoResponse}};

    #[test]
    fn message_test () {
        let mut app = App::default();

        pub fn contract_cw20() -> Box<dyn Contract<Empty>> {
            let contract = ContractWrapper::new(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            );
            Box::new(contract)
        }
        pub fn contract_cw721() -> Box<dyn Contract<Empty>> {
            let contract = ContractWrapper::new(
                cw721_base::entry::execute,
                cw721_base::entry::instantiate,
                cw721_base::entry::query,
            );
            Box::new(contract)
        }

        let owner_addr = Addr::unchecked("owner");
        let user_addr = Addr::unchecked("user");

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let cw20 = app.store_code(contract_cw20());
        let cw721 = app.store_code(contract_cw721());

        let cw20_addr = app.instantiate_contract(
            cw20, 
            owner_addr.clone(), 
            &cw20_base::msg::InstantiateMsg{
                name: "token".to_owned(),
                symbol: "tok".to_owned(),
                decimals: 6,
                initial_balances: vec![cw20::Cw20Coin{address: owner_addr.clone().to_string(), amount: Uint128::new(1000000)}],
                mint: Some(cw20::MinterResponse{
                    minter: owner_addr.to_string(),
                    cap: None,
                }),
                marketing: None,
            }, 
            &[], 
            "tokens", 
            None,
        ).unwrap();

        let cw721_addr = app.instantiate_contract(
            cw721, 
            owner_addr.clone(), 
            &cw721_base::InstantiateMsg {
                name: "nfts".to_owned(),
                symbol: "pbt".to_owned(),
                minter: owner_addr.clone().to_string(),
            }, 
            &[], 
            "nfts", 
            None,
        ).unwrap();

        
        let pb_addr = app.instantiate_contract(
            code_id, 
            owner_addr.clone(), 
            &InstantiateMsg{
                cw20_addr: cw20_addr.clone().to_string(),
                cw721_addr: cw721_addr.clone().to_string(),
                admin: owner_addr.clone().to_string(),
            }, 
            &[], 
            "pb", 
            None
        ).unwrap();

        let mintnft = app.execute_contract(
            owner_addr.clone(), 
            cw721_addr.clone(), 
            &cw721_base::ExecuteMsg::<Empty, Empty>::Mint { 
                token_id: "nft1".to_owned(), 
                owner: "owner".to_owned(), 
                token_uri: None, 
                extension: Empty {  }, 
            }, 
            &[],
        ).unwrap();

        let createpool = app.execute_contract(
            owner_addr.clone(), 
            pb_addr.clone(),
            &ExecuteMsg::CreatePool { price: Uint128::new(5) }, 
            &[]
        ).unwrap();

        let transferNft = app.execute_contract(
            owner_addr.clone(), 
            cw721_addr.clone(), 
            &cw721::Cw721ExecuteMsg::SendNft { 
                contract: pb_addr.clone().to_string(), 
                token_id: "nft1".to_owned(), 
                msg: to_binary(&RecieveNftMsg::AddNft { poolid: 0 }).unwrap(),
            }, 
            &[],
        ).unwrap();

        let sendtokens = app.execute_contract(
            owner_addr.clone(), 
            cw20_addr.clone(), 
            &cw20::Cw20ExecuteMsg::Transfer { recipient: user_addr.clone().to_string(), amount: Uint128::new(10) }, 
            &[] 
        ).unwrap();

        let redeemtoken = app.execute_contract(
            user_addr.clone(), 
            cw20_addr.clone(), 
            &cw20::Cw20ExecuteMsg::Send { 
                contract: pb_addr.clone().to_string(), 
                amount: Uint128::new(5), 
                msg: to_binary(&RecieveTokenMsg::RedeemToken { poolid: 0 }).unwrap()
            }, 
            &[],
        ).unwrap();

        let owner_balance : BalanceResponse = app.wrap().query_wasm_smart(cw20_addr.clone(), &cw20::Cw20QueryMsg::Balance { address: owner_addr.clone().to_string() }).unwrap();
        let pb_balance : BalanceResponse = app.wrap().query_wasm_smart(cw20_addr.clone(), &cw20::Cw20QueryMsg::Balance { address: pb_addr.clone().to_string() }).unwrap();
        let user_balance : BalanceResponse = app.wrap().query_wasm_smart(cw20_addr.clone(), &cw20::Cw20QueryMsg::Balance { address: user_addr.clone().to_string() }).unwrap();
        println!("owner: {:?}\npb: {:?}\nuser: {:?}", owner_balance, pb_balance, user_balance);

        let owner_nfts: TokensResponse = app.wrap().query_wasm_smart(cw721_addr.clone(), &cw721::Cw721QueryMsg::Tokens { owner: owner_addr.clone().to_string(), start_after: None, limit: None}).unwrap();
        let pb_nfts: TokensResponse = app.wrap().query_wasm_smart(cw721_addr.clone(), &cw721::Cw721QueryMsg::Tokens { owner: pb_addr.clone().to_string(), start_after: None, limit: None}).unwrap();
        let user_nfts: TokensResponse = app.wrap().query_wasm_smart(cw721_addr.clone(), &cw721::Cw721QueryMsg::Tokens { owner: user_addr.clone().to_string(), start_after: None, limit: None}).unwrap();
        println!("owner: {:?}\npb: {:?}\nuser: {:?}", owner_nfts, pb_nfts, user_nfts);

        let numpools: NumPoolsResponse = app.wrap().query_wasm_smart(pb_addr.clone(), &QueryMsg::NumPools {}).unwrap();
        let poolinfo: PoolInfoResponse = app.wrap().query_wasm_smart(pb_addr.clone(), &QueryMsg::PoolInfo { poolid: 0 }).unwrap();
        println!("numpools: {:?}\n poolinfo(0): {:?}",numpools,poolinfo);

        assert_eq!(1, 0)


    }
}