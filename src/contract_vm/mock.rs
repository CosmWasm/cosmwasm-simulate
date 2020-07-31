use cosmwasm_vm::{Api, Instance, Extern, call_query, call_handle};
use cosmwasm_vm::testing::{mock_env, MockApi, MockInstanceOptions};
use cosmwasm_std::{HandleResponse, WasmQuery,
                   QuerierResult, SystemError, CosmosMsg, WasmMsg, StdResult, HumanAddr, Coin, Env,};
use kv::{Config, Store};
use std::collections::{HashMap, BTreeMap};
use std::sync::Mutex;
// use rand::{thread_rng, Rng};
// use rand::distributions::Alphanumeric;

use crate::contract_vm::storage::MyMockStorage;
use crate::contract_vm::querier::{MyMockQuerier, CallBackFunc};

#[derive(Clone)]
struct ContractInfo {
    name: String,
    code: Vec<u8>,
}

lazy_static! {
    static ref TEST_STORE: Store = Store::new(Config::new("./tmp").temporary(true)).unwrap();
    static ref CONTRACT_INFO: Mutex<HashMap<HumanAddr, ContractInfo>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

pub fn mock_env_addr<A: Api>(_: &A, sender: &HumanAddr, _: &HumanAddr, sent: &[Coin]) -> Env {
    let env = mock_env(sender, sent);
    env
}

pub fn mock_instance<'a>(
    wasm: &[u8],
    contract_balance: &[Coin],
    contract_address: HumanAddr,
    contract_storage: MyMockStorage,
    call_back: CallBackFunc,
) -> Instance<MyMockStorage, MockApi, MyMockQuerier> {
    // check_wasm(wasm, &options.supported_features).unwrap();

    let options = MockInstanceOptions {
        contract_balance: Some(contract_balance),
        ..Default::default()
    };

    // merge balances
    let mut balances = options.balances.to_vec();
    if let Some(contract_balance) = options.contract_balance {
        // Remove old entry if exists
        if let Some(pos) = balances.iter().position(|item| *item.0 == contract_address) {
            balances.remove(pos);
        }
        balances.push((&contract_address, contract_balance));
    }

    let deps = Extern {
        api: MockApi::new(32),
        querier: MyMockQuerier::new(&balances, call_back),
        storage: contract_storage,
    };
    Instance::from_code(wasm, deps, options.gas_limit).unwrap()
}

pub fn install<'a>(contract_address: HumanAddr, contract_name: String, contract_code: Vec<u8>) -> Instance<MyMockStorage, MockApi, MyMockQuerier> {
    let contract_bucket = BTreeMap::new();
    let contract_store = MyMockStorage::new(contract_bucket);
    let contract_deps = mock_instance(contract_code.clone().as_slice(), &[],
                                          contract_address.clone(), contract_store, query_call_back);

    let mut contract_map = CONTRACT_INFO.lock().unwrap();
    contract_map.insert(contract_address.clone(), ContractInfo{
        name: contract_name,
        code: contract_code
    });
    // you must drop it here, or it will hold the lock and block the test process
    drop(contract_map);

    contract_deps
}

pub fn instantiate<'a>(contract_addr: HumanAddr) -> Instance<MyMockStorage, MockApi, MyMockQuerier> {
    let contract_map = CONTRACT_INFO.lock().unwrap();
    let contract_info = contract_map.get(&contract_addr.clone()).unwrap();

    let contract_bucket = BTreeMap::new();
    let contract_store = MyMockStorage::new(contract_bucket);
    let contract_deps = mock_instance(contract_info.code.as_slice(), &[],
                                          contract_addr, contract_store, query_call_back);

    // you must drop it here, or it will hold the lock and block the test process
    drop(contract_map);

    contract_deps
}

fn query_call_back(request: &WasmQuery) -> QuerierResult{
    match request{
        WasmQuery::Smart{ contract_addr, msg } => {
            let mut query_deps= instantiate(contract_addr.clone());
            let res = call_query(&mut query_deps, msg.as_slice()).unwrap();
            Ok(res)
        }
        _ =>{Err(SystemError::Unknown{} )}
        // WasmQuery::Raw{ contract_addr,key } => {
        //     Err(SystemError::NoSuchContract { addr: contract_addr.clone() })
        // }
    }
}

pub fn handler_resp(res:HandleResponse, caller: HumanAddr) -> StdResult<HandleResponse>{
    let msgs_itr = res.messages.iter();
    for msg in msgs_itr {
        match msg{
            CosmosMsg::Wasm(wasm_msg) => {
                match wasm_msg{
                    WasmMsg::Execute{ contract_addr, msg, send } => {
                        let mut handler_deps= instantiate(contract_addr.clone());
                        let env = mock_env_addr(&handler_deps.api, &caller, &contract_addr, send);
                        let res = call_handle(&mut handler_deps, &env, msg.as_slice()).unwrap().unwrap();

                        if res.messages.len() > 0 {
                            let _ = handler_resp(res, contract_addr.clone());
                        }
                    }
                    _ => {
                    }
                }
            }
            _ => {
            }
        }
    }

    Ok(HandleResponse::default())
}

// cosmwasm-std only support len(canonicalize_address) = 32 cache and len(HumanAddr) = 90 cache,
// but we need to change HumanAddress to canonicalize_address, so we only generate 32 len address.
// pub fn generate_address() -> HumanAddr{
//     let rand_string: String = thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(26)
//         .collect();
//
//     let mut address_prefix = "cosmos".to_string();
//     address_prefix += &rand_string.to_lowercase();
//
//     HumanAddr(address_prefix)
// }