use std::collections::BTreeMap;
#[cfg(feature = "iterator")]
use std::{
    iter,
    ops::{Bound, RangeBounds},
};


use cosmwasm_vm::{ReadonlyStorage, FfiResult, Storage, Api, FfiError, Extern};
use cosmwasm_std::{HumanAddr, CanonicalAddr, Binary, Coin};
use crate::contract_vm::watcher;

///mock storage
#[derive(Default, Debug)]
pub struct MockStorage {
    data: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl MockStorage {
    pub fn new() -> Self {
        MockStorage::default()
    }
}


impl ReadonlyStorage for MockStorage {
    fn get(&self, key: &[u8]) -> FfiResult<Option<Vec<u8>>> {
        Ok(self.data.get(key).cloned())
    }

    #[cfg(feature = "iterator")]
    fn range<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> FfiResult<Box<dyn Iterator<Item = FfiResult<KV>> + 'a>> {
        let bounds = range_bounds(start, end);

        // BTreeMap.range panics if range is start > end.
        // However, this cases represent just empty range and we treat it as such.
        match (bounds.start_bound(), bounds.end_bound()) {
            (Bound::Included(start), Bound::Excluded(end)) if start > end => {
                return Ok(Box::new(iter::empty()));
            }
            _ => {}
        }

        let iter = self.data.range(bounds);
        Ok(match order {
            Order::Ascending => Box::new(iter.map(clone_item).map(FfiResult::Ok)),
            Order::Descending => Box::new(iter.rev().map(clone_item).map(FfiResult::Ok)),
        })
    }
}

impl Storage for MockStorage {

    fn set(&mut self, key: &[u8], value: &[u8]) -> FfiResult<()> {
        self.data.insert(key.to_vec(), value.to_vec());
        watcher::logger_storage_event_insert(key,value);
        Ok(())
    }

    fn remove(&mut self, key: &[u8]) -> FfiResult<()> {
        self.data.remove(key);

        Ok(())
    }
}

impl MockStorage{

}

//mock api
#[derive(Copy, Clone)]
pub struct MockApi {
    canonical_length: usize,
}

impl MockApi {
    pub fn new(canonical_length: usize) -> Self {
        MockApi { canonical_length }
    }
}

impl Default for MockApi {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Api for MockApi {
    fn canonical_address(&self, human: &HumanAddr) -> FfiResult<CanonicalAddr> {
        // Dummy input validation. This is more sophisticated for formats like bech32, where format and checksum are validated.
        if human.len() < 3 {
            return Err(FfiError::other("Invalid input: human address too short"));
        }
        if human.len() > self.canonical_length {
            return Err(FfiError::other("Invalid input: human address too long"));
        }

        let mut out = Vec::from(human.as_str());
        let append = self.canonical_length - out.len();
        if append > 0 {
            out.extend(vec![0u8; append]);
        }
        Ok(CanonicalAddr(Binary(out)))
    }

    fn human_address(&self, canonical: &CanonicalAddr) -> FfiResult<HumanAddr> {
        if canonical.len() != self.canonical_length {
            return Err(FfiError::other(
                "Invalid input: canonical address length not correct",
            ));
        }

        // remove trailing 0's (TODO: fix this - but fine for first tests)
        let trimmed: Vec<u8> = canonical
            .as_slice()
            .iter()
            .cloned()
            .filter(|&x| x != 0)
            .collect();
        // decode UTF-8 bytes into string
        let human = String::from_utf8(trimmed)
            .map_err(|_| FfiError::other("Could not parse human address result as utf-8"))?;
        Ok(HumanAddr(human))
    }
}

pub fn new_mock(canonical_length: usize,
                contract_balance: &[Coin],
                contract_addr : &str
) -> Extern<MockStorage,MockApi,cosmwasm_vm::testing::MockQuerier>{
    let human_addr = HumanAddr::from(contract_addr);
    Extern {
        storage: MockStorage::default(),
        api: MockApi::new(canonical_length),
        querier: cosmwasm_vm::testing::MockQuerier::new(&[(&human_addr, contract_balance)]),
    }
}