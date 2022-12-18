use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry<K: Ord + Clone + Copy + Display, V: Clone + Copy + Display> {

    pub key: K,

    pub value: V,
}


impl <K: Ord + Clone + Copy + Display, V: Clone + Copy + Display> Entry<K, V> {
    pub fn new(key: K, value: V) -> Entry<K, V> {
        Self {
            key,
            value,
        }
    }
}

impl <K: Ord + Clone + Copy + Display, V: Clone + Copy + Display> Display for Entry<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        println!("{}-{}", self.key, self.value);
        Ok(())
    }
}
