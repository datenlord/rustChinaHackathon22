mod entry;
mod lockfree_queue;
mod skip;
mod atomic;

use std::{collections::BTreeMap, sync::Mutex};
use std::collections::LinkedList;
use std::iter::Skip;
use crate::entry::Entry;
use crate::lockfree_queue::Queue;
use crate::skip::SkipList;


/// Operations of Index
pub(crate) trait IndexOperate<K: Ord, V> {
    /// Get a range of keys in [key, range_end]
    fn get(&self, key: &K, range_end: &K) -> Vec<&V>;
    /// delete a range of keys in [key, range_end]     
    fn delete(&self, key: &K, range_end: &K) -> Vec<V>;
    /// insert of update a key     
    fn insert_or_update(&self, key: K, value: V) -> Option<V>;
}

pub struct KVStore<K: Ord, V> {
    map: Mutex<BTreeMap<K, V>>,
}

pub struct KVQueue<K: Ord, V> {
    queue: Queue<Entry<K, V>>,
}

impl <K: Ord, V> KVQueue<K, V> {
    pub fn new() -> Self {
        Self {
            queue: Queue::new(),
        }
    }
}


impl<K: Ord, V> KVStore<K, V> {
    pub fn new() -> Self {
        Self {
            map: Mutex::new(BTreeMap::new()),
        }
    }
}

impl<K: Ord, V> IndexOperate<K, V> for KVQueue<K, V> {
    fn get(&self, key: &K, range_end: &K) -> Vec<&V> {
        todo!()
    }

    fn delete(&self, key: &K, range_end: &K) -> Vec<V> {
        todo!()
    }

    fn insert_or_update(&self, key: K, value: V) -> Option<V> {
        let entry = Entry::new(key, value);
        self.queue.push(entry);
        // thread::spawn( move || {
        // let entry = self.queue.pop().unwrap();
        // self.list.insert_or_update(entry.key, entry.value);
        //
        // });

        None
    }
}

impl<K: Ord, V> IndexOperate<K, V> for KVStore<K, V> {
    fn get(&self, key: &K, range_end: &K) -> Vec<&V> {
        todo!()
    }

    fn delete(&self, key: &K, range_end: &K) -> Vec<V> {
        todo!()
    }

    fn insert_or_update(&self, key: K, value: V) -> Option<V> {
        let mut lock = self.map.lock().unwrap();
        lock.insert(key, value);
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get() {}
}
