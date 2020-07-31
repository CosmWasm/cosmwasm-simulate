use cosmwasm_std::{HumanAddr, Coin, Querier as stdQuerier, WasmQuery, to_binary, QueryRequest, QuerierResult as stdQuerierResult, SystemError, from_slice, Empty};
use serde::Serialize;
use cosmwasm_std::testing::{BankQuerier, StakingQuerier};
use cosmwasm_vm::{GasInfo, Querier};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub type CallBackFunc = fn(&WasmQuery) -> stdQuerierResult;

const GAS_COST_QUERY_FLAT: u64 = 100_000;
/// Gas per request byte
const GAS_COST_QUERY_REQUEST_MULTIPLIER: u64 = 0;
/// Gas per reponse byte
const GAS_COST_QUERY_RESPONSE_MULTIPLIER: u64 = 100;


#[derive(Clone)]
pub struct MyMockQuerier {
    querier: StdMockQuerier,
}

impl MyMockQuerier {
    pub fn new(balances: &[(&HumanAddr, &[Coin])], call_back: CallBackFunc) -> Self {
        MyMockQuerier {
            querier: StdMockQuerier::new(balances,  call_back),
        }
    }

    // set a new balance for the given address and return the old balance
    pub fn update_balance<U: Into<HumanAddr>>(
        &mut self,
        addr: U,
        balance: Vec<Coin>,
    ) -> Option<Vec<Coin>> {
        self.querier.update_balance(addr, balance)
    }

    #[cfg(feature = "staking")]
    pub fn with_staking(
        &mut self,
        denom: &str,
        validators: &[cosmwasm_std::Validator],
        delegations: &[cosmwasm_std::FullDelegation],
    ) {
        self.querier.with_staking(denom, validators, delegations);
    }
}


impl cosmwasm_vm::Querier for MyMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_vm::QuerierResult {
        let res = self.querier.raw_query(bin_request);
        // We don't use FFI, so FfiResult is always Ok() regardless of error on other levels
        let gas_info = GasInfo::with_externally_used(
            GAS_COST_QUERY_FLAT
                + (GAS_COST_QUERY_REQUEST_MULTIPLIER * (2))
                + (GAS_COST_QUERY_RESPONSE_MULTIPLIER),
        );

        return (Ok(res),gas_info);
    }
}

impl MyMockQuerier {
    pub fn handle_query<T: Serialize>(&self, request: &QueryRequest<T>) -> cosmwasm_vm::QuerierResult {
        // encode the request, then call raw_query

        let gas_info = GasInfo::with_externally_used(
            GAS_COST_QUERY_FLAT
                + (GAS_COST_QUERY_REQUEST_MULTIPLIER * (2))
                + (GAS_COST_QUERY_RESPONSE_MULTIPLIER),
        );

        let bin = match to_binary(request) {
            Ok(raw) => raw,
            Err(e) => {

                let res = Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: b"N/A".into(),}
                    );
                return (Ok(res),gas_info);
            }
        };
        self.raw_query(bin.as_slice())
    }
}


#[derive(Clone, Default)]
pub struct StdMockQuerier<C: DeserializeOwned = Empty>{
    bank: BankQuerier,
    staking: StakingQuerier,
    wasm: CustomizationWasmQuerier,
    marker :PhantomData<C>
}

impl<C: DeserializeOwned> StdMockQuerier<C> {
    pub fn new(balances: &[(&HumanAddr, &[Coin])], call_back: CallBackFunc) -> Self {
        StdMockQuerier {
            bank: BankQuerier::new(balances),
            staking: StakingQuerier::default(),
            wasm: CustomizationWasmQuerier::new(call_back),
            marker: PhantomData,
        }
    }

    // set a new balance for the given address and return the old balance
    pub fn update_balance<U: Into<HumanAddr>>(
        &mut self,
        _addr: U,
        _balance: Vec<Coin>,
    ) -> Option<Vec<Coin>> {
        None
    }

    #[cfg(feature = "staking")]
    pub fn with_staking(
        &mut self,
        denom: &str,
        validators: &[crate::query::Validator],
        delegations: &[crate::query::FullDelegation],
    ) {
        self.staking = StakingQuerier::new(denom, validators, delegations);
    }

    pub fn handle_query<T>(&self, request: &QueryRequest<T>) -> cosmwasm_std::QuerierResult {
        match &request {
            QueryRequest::Bank(bank_query) => self.bank.query(bank_query),
            QueryRequest::Custom(_) => Err(SystemError::UnsupportedRequest {
                kind: "custom".to_string(),
            }),
            QueryRequest::Staking(staking_query) => self.staking.query(staking_query),
            QueryRequest::Wasm(msg) => self.wasm.query(msg),
        }
    }
}

impl <C: DeserializeOwned> stdQuerier for StdMockQuerier<C>{
    fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request:QueryRequest<C> = match from_slice(&bin_request) {
            Ok(v) => v,
            Err(e) => {
                return Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        return self.handle_query(&request)
    }
}

#[derive(Clone)]
struct CustomizationWasmQuerier {
    call_back: CallBackFunc,
}

impl Default for CustomizationWasmQuerier{
    fn default() -> Self {
        CustomizationWasmQuerier { call_back: default_call_back}
    }
}

pub fn default_call_back(_request: &WasmQuery) -> stdQuerierResult {
    Err(SystemError::UnsupportedRequest { kind: "Wasm query".to_string() })
}

impl CustomizationWasmQuerier {
    pub fn new(call_back: CallBackFunc) -> Self {
        CustomizationWasmQuerier {call_back }
    }

    fn query(&self, request: &WasmQuery) -> stdQuerierResult {
        (self.call_back)(request)
    }
}