#![feature(step_trait)]
#![allow(unused)]
use std::iter::Step;

pub mod index;

mod consts;
mod hash_bucket;

/// Operations of Index
pub(crate) trait IndexOperate<K, V> {
    /// Get a range of keys in [key, range_end]
    fn get(&self, key: &K, range_end: &K) -> Vec<V>;
    /// delete a range of keys in [key, range_end]
    fn delete(&self, key: &K, range_end: &K) -> Vec<V>;
    /// insert of update a key
    fn insert_or_update(&self, key: K, value: V) -> Option<V>; 
}

pub mod crossbeam_gate {
    use super::*;
    use crossbeam_skiplist::SkipMap;

    /// KV store inner
    /// using [crossbeam-skiplist](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-skiplist) to back it up
    pub(crate) struct Index<K, V> {
        /// index
        index: SkipMap<K, V>
    }

    impl<K, V> IndexOperate<K, V> for Index<K, V> 
    where 
        K: Ord + Send + Step + Clone + 'static,
        V: Send + Clone + 'static
    {
        fn get(&self, key: &K, range_end: &K) -> Vec<V> {
            self.index
                .range(key..=range_end)
                .map(|entry| entry.value().clone())
                .collect::<Vec<_>>()
        }

        fn delete(&self, key: &K, range_end: &K) -> Vec<V> {
            (key.clone()..=range_end.clone())
                .map(|ref k| {
                    self.index.remove(k)
                })
                .take_while(|ret| ret.is_some())
                .map(|ret| ret.unwrap().value().clone())
                .collect::<Vec<_>>()
        }

        fn insert_or_update(&self, key: K, value: V) -> Option<V> {
            let entry = self.index
                .get_or_insert(key, value);
            let ret = entry.value();
            
            Some(ret.to_owned())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::Barrier;
        use crossbeam_utils::thread;

        // https://github.com/crossbeam-rs/crossbeam/issues/672
        #[test]
        fn concurrent_insert() {
            for _ in 0..100 {
                let index: Index<i32, i32> = Index { index: SkipMap::new() };
                let barrier = Barrier::new(2);
                thread::scope(|s| {
                    s.spawn(|_| {
                        barrier.wait();
                        index.insert_or_update(1, 1);
                    });
                    s.spawn(|_| {
                        barrier.wait();
                        index.insert_or_update(1, 1);
                    });
                })
                .unwrap();
            }
        }

        // https://github.com/crossbeam-rs/crossbeam/issues/672
        #[test]
        fn concurrent_remove() {
            for _ in 0..100 {
                let index: Index<i32, i32> = Index { index: SkipMap::new() };
                let barrier = Barrier::new(2);
                thread::scope(|s| {
                    s.spawn(|_| {
                        barrier.wait();
                        index.delete(&1, &2);
                    });
                    s.spawn(|_| {
                        barrier.wait();
                        index.delete(&1, &2);
                    });
                })
                .unwrap();
            }
        }
    }
}

mod faster_gate {
    use super::*;
    use faster::{FasterKv, FasterKey, FasterValue};

    /// KV store inner
    /// using [crossbeam-skiplist](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-skiplist) to back it up
    pub(crate) struct Index {
        /// index
        index: FasterKv
    }

    // TODO: implement faster kv store in pure rust and design a compatible API
}