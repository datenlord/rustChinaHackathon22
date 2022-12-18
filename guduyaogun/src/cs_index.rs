use crossbeam_skiplist::SkipMap;
use std::sync::Arc;

use crate::Index;
use crate::IndexOperate;

/// crossbeam-skiplist
#[derive(Clone)]
pub struct CSIndex<K: Ord, V> {
    inner: Arc<SkipMap<K, V>>,
}

impl<K: Ord, V> Index for CSIndex<K, V> {
    fn new() -> Self {
        Self {
            inner: Arc::new(SkipMap::new()),
        }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V> IndexOperate<K, V> for CSIndex<K, V>
where
    K: Ord + Send + Clone + 'static,
    V: Send + Clone + 'static,
{
    fn get(&self, key: &K, range_end: &K) -> Vec<&V> {
        self.inner
            .range(key..=range_end)
            .filter_map(|entry| Some(unsafe { &*(entry.value() as *const V) }))
            .collect()
    }

    fn delete(&self, key: &K, range_end: &K) -> Vec<V> {
        self.inner
            .range(key..=range_end)
            .filter_map(|entry| {
                let rst = entry.value().clone();
                entry.remove();
                Some(rst)
            })
            .collect()
    }

    fn insert_or_update(&self, key: K, value: V) -> Option<V> {
        let rst = self.inner.get(&key).and_then(|entry| {
            let rst = entry.value().clone();
            entry.remove();
            Some(rst)
        });
        let _entry = self.inner.insert(key, value);
        rst
    }
}
