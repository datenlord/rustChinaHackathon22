use crate::IndexOperate;
use crossbeam_skiplist::SkipMap;
use std::cell::UnsafeCell;

/// KV store inner
#[derive(Debug)]
pub struct Index<K: Ord, V> {
    /// index
    index: SkipMap<K, UnsafeCell<Option<V>>>,
}

impl<K: Ord, V> Index<K, V> {
    pub fn new() -> Self {
        Self {
            index: SkipMap::new(),
        }
    }
}

impl<K: Ord, V> Default for Index<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<K: Ord + Send + 'static, V: Send + 'static> Sync for Index<K, V> {}

impl<K: Ord + Send + 'static, V: Send + 'static> IndexOperate<K, V> for Index<K, V> {
    /// Get a range of keys in [start, end]
    fn get(&self, start: &K, end: &K) -> Vec<&V> {
        if start == end {
            self.index
                .get(start)
                .map(|entry| {
                    let v = entry.value();
                    vec![unsafe { v.get().as_ref().unwrap().as_ref().unwrap() }]
                })
                .unwrap_or_default()
        } else {
            self.index
                .range(start..=end)
                .map(|entry| {
                    let v = entry.value();
                    unsafe { v.get().as_ref().unwrap().as_ref().unwrap() }
                })
                .collect()
        }
    }
    /// delete a range of keys in [start, end]
    fn delete(&self, start: &K, end: &K) -> Vec<V> {
        if start == end {
            self.index
                .remove(start)
                .map(|entry| {
                    let v = entry.value();
                    let v = v.get();
                    let mut ret = None;
                    unsafe {
                        std::ptr::swap(&mut ret as *mut Option<V>, v);
                        vec![ret.unwrap()]
                    }
                })
                .unwrap_or_default()
        } else {
            self.index
                .range(start..=end)
                .filter_map(|entry| {
                    self.index.remove(entry.key()).map(|entry| {
                        let v = entry.value();
                        let v = v.get();
                        let mut ret = None;
                        unsafe {
                            std::ptr::swap(&mut ret as *mut Option<V>, v);
                            ret.unwrap()
                        }
                    })
                })
                .collect()
        }
    }
    /// insert of update a key
    fn insert_or_update(&self, key: K, value: V) -> Option<V> {
        let before = self.index.get(&key);
        self.index.insert(key, UnsafeCell::new(Some(value)));
        before.map(|entry| {
            let v = entry.value();
            let v = v.get();
            let mut ret = None;
            unsafe {
                std::ptr::swap(&mut ret as *mut Option<V>, v);
                ret.unwrap()
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::super::test;
    use super::Index;
    #[test]
    fn test() {
        test::basic(|| Index::new());
    }

    #[test]
    fn test_multithread() {
        test::multithread(|| Index::new());
    }
}
