use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

use crate::Index;
use crate::IndexOperate;

/// Most common and simple index: Mutex + BplusTree.
#[derive(Clone)]
pub struct SimpleIndex<K: Ord, V> {
    inner: Arc<Mutex<BTreeMap<K, V>>>,
}

impl<K: Ord, V> Index for SimpleIndex<K, V> {
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }
}

impl<K: Ord, V> IndexOperate<K, V> for SimpleIndex<K, V> {
    fn get(&self, key: &K, range_end: &K) -> Vec<&V> {
        let index = self.inner.lock().unwrap();
        index
            .range(key..=range_end)
            .filter_map(|(_k, v)| Some(unsafe { &*(v as *const V) }))
            .collect()
    }

    fn delete(&self, key: &K, range_end: &K) -> Vec<V> {
        let mut index = self.inner.lock().unwrap();
        let keys: Vec<&K> = index
            .range(key..=range_end)
            .filter_map(|(k, _v)| Some(unsafe { &*(k as *const K) }))
            .collect();
        let mut rst = Vec::new();
        for k in keys {
            if let Some(v) = index.remove(k) {
                rst.push(v);
            }
        }
        rst
    }

    fn insert_or_update(&self, key: K, value: V) -> Option<V> {
        let mut index = self.inner.lock().unwrap();
        index.insert(key, value)
    }
}
