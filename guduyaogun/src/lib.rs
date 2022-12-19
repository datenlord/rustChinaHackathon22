mod cs_index;
mod simple;
mod skipmap;

pub use cs_index::CSIndex;
pub use simple::SimpleIndex;
pub use skipmap::SkipMapIndex;

pub trait Index {
    fn new() -> Self;
    fn len(&self) -> usize;
}

/// Operations of Index
pub trait IndexOperate<K: Ord, V> {
    /// Get a range of keys in [key, range_end]
    fn get(&self, key: &K, range_end: &K) -> Vec<&V>;
    /// delete a range of keys in [key, range_end]     
    fn delete(&self, key: &K, range_end: &K) -> Vec<V>;
    /// insert of update a key     
    fn insert_or_update(&self, key: K, value: V) -> Option<V>;
}
