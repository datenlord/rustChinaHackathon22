use super::SkipMap;
use std::sync::Arc;
use std::sync::Mutex;

use crate::Index;
use crate::IndexOperate;

/// skip-map
#[derive(Clone)]
pub struct SkipMapIndex<K: Ord, V> {
    inner: Arc<SkipMap<K, V>>,
    n: Arc<Mutex<usize>>,
}

impl<K: Ord + Default, V: Default> Index for SkipMapIndex<K, V> {
    fn new() -> Self {
        Self {
            inner: Arc::new(SkipMap::new()),
            n: Arc::new(Mutex::new(0)),
        }
    }

    fn len(&self) -> usize {
        *self.n.lock().unwrap()
    }
}

impl<K, V> IndexOperate<K, V> for SkipMapIndex<K, V>
where
    K: Ord + Send + Clone + 'static,
    V: Send + Clone + 'static,
{
    fn get(&self, key: &K, range_end: &K) -> Vec<&V> {
        self.inner.get_range(key, range_end)
    }

    fn delete(&self, key: &K, range_end: &K) -> Vec<V> {
        let ret = self.inner.del_range(key, range_end);
        let mut n = self.n.lock().unwrap();
        *n -= ret.len();
        ret
    }

    fn insert_or_update(&self, key: K, value: V) -> Option<V> {
        let mut n = self.n.lock().unwrap();
        let ret = self.inner.insert_or_update(key, value);
        if ret.is_none() {
            *n += 1;
        }
        ret
    }
}
