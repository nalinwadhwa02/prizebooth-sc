#[cfg(test)]
pub mod tests {
    use my_cw20;
    use my_cw721;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Response, to_binary, Uint128, Addr, Empty, from_binary, CosmosMsg, WasmMsg};
    use cw20::{Cw20ReceiveMsg, Balance, Cw20Coin, MinterResponse};
    use cw_multi_test::{App, ContractWrapper, Executor};
    use crate::state::Prizepool;
    use crate::{execute, instantiate, query};
    use crate::msg::{ReceiveTokenMsg, ExecuteMsg, InstantiateMsg, ReceiveNftMsg, QueryMsg};

    #[test]

    #[test]
    fn multi_contract_test () {
        let mut app = App::default();

        //upload contracts
        let pb_code = ContractWrapper::new(execute, instantiate, query);
        let pb_code_id = app.store_code(Box::new(pb_code));

        let token_code = ContractWrapper::new(my_cw20::contract::execute, my_cw20::contract::instantiate, my_cw20::contract::query);
        let token_code_id = app.store_code(Box::new(token_code));

        let nft_code = ContractWrapper::new(my_cw721::entry::execute, my_cw721::entry::instantiate, my_cw721::entry::query);
        let nft_code_id = app.store_code(Box::new(nft_code));

        //instantiating contracts
        let tokentract_addr = app.instantiate_contract(
            token_code_id, 
            Addr::unchecked("owner"), 
            &my_cw20::msg::InstantiateMsg{
                name: "pb_token".to_owned(),
                symbol: "pbt".to_owned(),
                decimals: 6,
                initial_balances: vec![Cw20Coin {address: "owner".to_owned(), amount: Uint128::new(10000000)}],
                mint: Some(MinterResponse{
                    minter: "owner".to_owned(),
                    cap: None,
                }),
                marketing: None
            }, 
            &[], 
            "token_contract".to_owned(), 
            Some("owner".to_owned())
        ).unwrap();

        let nfttract_addr = app.instantiate_contract(
            nft_code_id, 
            Addr::unchecked("owner"), 
            &my_cw721::msg::InstantiateMsg{
                name: "pb_nft".to_owned(),
                symbol: "pbn".to_owned(),
                minter: "owner".to_owned(),
            }, 
            &[], 
            "nft_contract".to_owned(), 
            Some("owner".to_owned())
        ).unwrap();

        let pbtract_addr = app.instantiate_contract(
            pb_code_id, 
            Addr::unchecked("owner"), 
            &InstantiateMsg{
                cw20_addr: tokentract_addr.to_string(),
                cw721_addr: nfttract_addr.to_string(),
            }, 
            &[], 
            "pb_contract".to_owned(), 
            Some("owner".to_owned())
        ).unwrap();

        let contractminter_resp = app.execute_contract(
            Addr::unchecked("owner"),
            tokentract_addr.clone(), 
            &my_cw20::msg::ExecuteMsg::UpdateMinter { new_minter: Some(pbtract_addr.clone().to_string()) }, 
            &[]
        ).unwrap();

        let crp_resp = app.execute_contract(
            Addr::unchecked("owner"), 
            pbtract_addr.clone(), 
            &ExecuteMsg::CreatePrizePool { mintprice: Uint128::new(5) }, 
            &[]
        ).unwrap();

        let mintnft_resp = app.execute_contract(
            Addr::unchecked("owner"), 
            nfttract_addr.clone(), 
            &my_cw721::msg::ExecuteMsg::<Empty,Empty>::Mint { 
                token_id: "nft1".to_owned(), 
                owner: "owner".to_owned(), 
                token_uri: None, 
                extension: Empty::default(),
            }, 
            &[]
        ).unwrap();

        let anft_resp = app.execute_contract(
            Addr::unchecked("owner"), 
            nfttract_addr.clone(), 
            &my_cw721::msg::ExecuteMsg::<Empty,Empty>::SendNft { 
                contract: pbtract_addr.to_string(), 
                token_id: "nft1".to_owned(), 
                msg: to_binary(&ReceiveNftMsg::AddNft { 
                    poolid: 0 }
                ).unwrap()
            },
            &[]
        ).unwrap();

        let poollistresp: Vec<Prizepool>  = app.wrap().query_wasm_smart(pbtract_addr.clone(), &QueryMsg::PoolList {  }).unwrap();
        assert_eq!(poollistresp[0].admin, "owner");
        assert_eq!(poollistresp[0].nft_list, vec!["nft1"]);

        let nftownerresp: cw721::OwnerOfResponse = app.wrap().query_wasm_smart(nfttract_addr.clone(), &my_cw721::msg::QueryMsg::<Empty>::OwnerOf { token_id: "nft1".to_owned(), include_expired: None }).unwrap();
        assert_eq!(nftownerresp.owner, pbtract_addr.to_string());

        let tokentrans = app.execute_contract(
            Addr::unchecked("owner"), 
            tokentract_addr.clone(), 
            &my_cw20::msg::ExecuteMsg::Transfer { 
                recipient: "minter".to_owned(), 
                amount: Uint128::new(100), 
            },
            &[]
        ).unwrap();

        let mintresp = app.execute_contract(
            Addr::unchecked("minter"),
            tokentract_addr.clone(), 
            &my_cw20::msg::ExecuteMsg::Send { 
                contract: pbtract_addr.to_string(), 
                amount: Uint128::new(5), 
                msg: to_binary(&ReceiveTokenMsg::Mint { poolid: 0 }).unwrap()
            }, 
            &[]
        );

        let tokenquery: cw20::BalanceResponse = app.wrap().query_wasm_smart(tokentract_addr.clone(), &my_cw20::msg::QueryMsg::Balance { address: "minter".to_owned() }).unwrap();
        assert_eq!(tokenquery.balance, Uint128::new(95));

        let tokenquery: cw20::BalanceResponse = app.wrap().query_wasm_smart(tokentract_addr, &my_cw20::msg::QueryMsg::Balance { address: "owner".to_owned() }).unwrap();
        assert_eq!(tokenquery.balance, Uint128::new(9999905));

        let poollistresp: Vec<Prizepool>  = app.wrap().query_wasm_smart(pbtract_addr.clone(), &QueryMsg::PoolList {  }).unwrap();
        assert_eq!(poollistresp[0].admin, "owner");
        // assert_eq!(poollistresp[0].nft_list, Vec::<String>::new());

        let nftownerresp: cw721::OwnerOfResponse = app.wrap().query_wasm_smart(nfttract_addr, &my_cw721::msg::QueryMsg::<Empty>::OwnerOf { token_id: "nft1".to_owned(), include_expired: None }).unwrap();
        assert_eq!(nftownerresp.owner, "minter".to_owned());


    }

}