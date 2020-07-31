use cosmwasm_vm::{FfiResult, GasInfo};

#[cfg(feature = "iterator")]
use kv::{Item, Error};

#[cfg(feature = "iterator")]
use cosmwasm_std::{Order};

use crate::contract_vm::watcher;
use std::collections::BTreeMap;

/// KV is a Key-Value pair, returned from our iterators
pub type KV = (Vec<u8>, Vec<u8>);

pub struct MyMockStorage {
    bucket: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl MyMockStorage {
    pub fn new(bucket: BTreeMap<Vec<u8>, Vec<u8>>) -> Self {
        MyMockStorage{bucket }
    }
}

#[cfg(feature = "iterator")]
fn clone_item(item: Result<Item<Raw, Raw>, Error>) -> KV {
    let itr = item.unwrap();

    let key = itr.key::<Raw>().unwrap().clone().to_vec();
    let value = itr.value::<Raw>().unwrap().clone().to_vec();

    (key, value)
}

impl<'a> Default for MyMockStorage{
    fn default() -> Self {
        let bucket = BTreeMap::new();
        MyMockStorage{bucket}
    }
}

impl<'a> cosmwasm_vm::Storage for MyMockStorage {

    fn get(&self, key: &[u8]) -> FfiResult<Option<Vec<u8>>> {
        let gas_info = GasInfo{cost:100,externally_used:200};
        return (Ok(self.bucket.get(key).cloned()),gas_info);
    }

    #[cfg(feature = "iterator")]
    /// range allows iteration over a set of keys, either forwards or backwards
    /// uses standard rust range notation, and eg db.range(b"foo"..b"bar") also works reverse
    fn range<'a>(
        &'a self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> FfiResult<Box<dyn Iterator<Item = KV> + 'a>> {
        let bounds = range_bounds(start, end);

        // BTreeMap.range panics if range is start > end.
        // However, this cases represent just empty range and we treat it as such.
        match (bounds.start_bound(), bounds.end_bound()) {
            (Bound::Included(start), Bound::Excluded(end)) if start > end => {
                return (Ok(Box::new(iter::empty())),GasInfo{cost:100,externally_used:200});
            }
            _ => {}
        }

        let iter = self.data.range(bounds);
        match order {
            Order::Ascending => (OkBox::new(iter.map(clone_item)),GasInfo{cost:100,externally_used:200}),
            Order::Descending => (OkBox::new(iter.rev().map(clone_item)),GasInfo{cost:100,externally_used:200}),
        }
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> FfiResult<()>{
        let _ = self.bucket.insert(key.to_vec(), value.to_vec());
        // self.bucket.flush();
        watcher::logger_storage_event_insert(key,value);
        return (Ok(()),GasInfo{cost:100,externally_used:200});

    }

    fn remove(&mut self, key: &[u8]) -> FfiResult<()> {
        let _ = self.bucket.remove(key);
        // self.bucket.flush();
        watcher::logger_storage_event_remove(key);
        return (Ok(()),GasInfo{cost:100,externally_used:200});
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "iterator")]
    // iterator_test_suite takes a storage, adds data and runs iterator tests
    // the storage must previously have exactly one key: "foo" = "bar"
    // (this allows us to test StorageTransaction and other wrapped storage better)
    fn iterator_test_suite(store: &mut MyMockStorage) {
        // ensure we had previously set "foo" = "bar"
        assert_eq!(store.get(b"foo").unwrap(), Some(b"bar".to_vec()));
        assert_eq!(
            store.range(None, None, Order::Ascending).unwrap().count(),
            1
        );

        // setup - add some data, and delete part of it as well
        store.set(b"ant", b"hill").expect("error setting value");
        store.set(b"ze", b"bra").expect("error setting value");

        // noise that should be ignored
        store.set(b"bye", b"bye").expect("error setting value");
        store.remove(b"bye").expect("error removing key");

        // unbounded
        {
            let iter = store.range(None, None, Order::Ascending).unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(
                elements,
                vec![
                    (b"ant".to_vec(), b"hill".to_vec()),
                    (b"foo".to_vec(), b"bar".to_vec()),
                    (b"ze".to_vec(), b"bra".to_vec()),
                ]
            );
        }

        // unbounded (descending)
        {
            let iter = store.range(None, None, Order::Descending).unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(
                elements,
                vec![
                    (b"ze".to_vec(), b"bra".to_vec()),
                    (b"foo".to_vec(), b"bar".to_vec()),
                    (b"ant".to_vec(), b"hill".to_vec()),
                ]
            );
        }

        // bounded
        {
            let iter = store
                .range(Some(b"f"), Some(b"n"), Order::Ascending)
                .unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(elements, vec![(b"foo".to_vec(), b"bar".to_vec())]);
        }

        // bounded (descending)
        {
            let iter = store
                .range(Some(b"air"), Some(b"loop"), Order::Descending)
                .unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(
                elements,
                vec![
                    (b"foo".to_vec(), b"bar".to_vec()),
                    (b"ant".to_vec(), b"hill".to_vec()),
                ]
            );
        }

        // bounded empty [a, a)
        {
            let iter = store
                .range(Some(b"foo"), Some(b"foo"), Order::Ascending)
                .unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(elements, vec![]);
        }

        // bounded empty [a, a) (descending)
        {
            let iter = store
                .range(Some(b"foo"), Some(b"foo"), Order::Descending)
                .unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(elements, vec![]);
        }

        // bounded empty [a, b) with b < a
        {
            let iter = store
                .range(Some(b"z"), Some(b"a"), Order::Ascending)
                .unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(elements, vec![]);
        }

        // bounded empty [a, b) with b < a (descending)
        {
            let iter = store
                .range(Some(b"z"), Some(b"a"), Order::Descending)
                .unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(elements, vec![]);
        }

        // right unbounded
        {
            let iter = store.range(Some(b"f"), None, Order::Ascending).unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(
                elements,
                vec![
                    (b"foo".to_vec(), b"bar".to_vec()),
                    (b"ze".to_vec(), b"bra".to_vec()),
                ]
            );
        }

        // right unbounded (descending)
        {
            let iter = store.range(Some(b"f"), None, Order::Descending).unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(
                elements,
                vec![
                    (b"ze".to_vec(), b"bra".to_vec()),
                    (b"foo".to_vec(), b"bar".to_vec()),
                ]
            );
        }

        // left unbounded
        {
            let iter = store.range(None, Some(b"f"), Order::Ascending).unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(elements, vec![(b"ant".to_vec(), b"hill".to_vec()),]);
        }

        // left unbounded (descending)
        {
            let iter = store.range(None, Some(b"no"), Order::Descending).unwrap();
            let elements: Vec<KV> = iter.filter_map(FfiResult::ok).collect();
            assert_eq!(
                elements,
                vec![
                    (b"foo".to_vec(), b"bar".to_vec()),
                    (b"ant".to_vec(), b"hill".to_vec()),
                ]
            );
        }
    }

    #[test]
    fn get_and_set() {
        let mut store = MyMockStorage::default();

        assert_eq!(None, store.get(b"foo").unwrap());
        store.set(b"foo", b"bar").unwrap();
        assert_eq!(Some(b"bar".to_vec()), store.get(b"foo").unwrap());
        assert_eq!(None, store.get(b"food").unwrap());


        // test different bucket with same bucket name are the same!
        let bucket2 = data.bucket::<Raw, Raw>(Some("test1")).unwrap();
        let mut store2 = MyMockStorage::new(bucket2);
        assert_eq!(Some(b"bar".to_vec()), store2.get(b"foo").unwrap());
    }

    #[test]
    fn delete() {
        let mut store = MyMockStorage::default();

        store.set(b"foo", b"bar").unwrap();
        store.set(b"food", b"bank").unwrap();
        store.remove(b"foo").unwrap();

        assert_eq!(None, store.get(b"foo").unwrap());
        assert_eq!(Some(b"bank".to_vec()), store.get(b"food").unwrap());
    }

    #[test]
    #[cfg(feature = "iterator")]
    fn iterator() {
        let mut store = MyMockStorage::default();

        store.set(b"foo", b"bar").expect("error setting value");
        iterator_test_suite(&mut store);
    }
}