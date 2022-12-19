use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry<K: Ord, V> {
    pub key: K,

    pub value: V,
}

impl<K: Ord, V> Entry<K, V> {
    pub fn new(key: K, value: V) -> Entry<K, V> {
        Self { key, value }
    }
}

// impl<K: Ord, V> Display for Entry<K, V> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         println!("{}-{}", self.key, self.value);
//         Ok(())
//     }
// }
