// code from https://github.com/datenlord/Xline
// to satisfy the trait IndexOperate, it's greatly modified.

use crate::IndexOperate;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::ops::Bound::Included;
use std::ptr;

/// KV store inner
#[derive(Debug)]
pub struct Index<K: Ord, V> {
    /// index
    index: Mutex<BTreeMap<K, *mut V>>,
}

impl<K: Ord, V> Index<K, V> {
    pub fn new() -> Self {
        Self {
            index: Mutex::new(BTreeMap::new()),
        }
    }
}

impl<K: Ord, V> Default for Index<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> IndexOperate<K, V> for Index<K, V> {
    fn get(&self, start: &K, end: &K) -> Vec<&V> {
        let index = self.index.lock();
        match RangeType::get_range_type(&start, &end) {
            RangeType::OneKey(..) => index
                .get(start)
                .map(|v| unsafe {
                    match v.as_ref() {
                        Some(v) => vec![v],
                        None => vec![],
                    }
                })
                .unwrap_or_default(),
            RangeType::Range(..) => index
                .range((Included(start), Included(end)))
                .filter_map(|(_k, v)| unsafe { v.as_ref() })
                .collect(),
        }
    }

    fn delete(&self, start: &K, end: &K) -> Vec<V> {
        let mut index = self.index.lock();
        match RangeType::get_range_type(&start, &end) {
            RangeType::OneKey(..) => index
                .remove(start)
                .map(|v| {
                    if v.is_null() {
                        vec![]
                    } else {
                        unsafe { vec![*Box::from_raw(v)] }
                    }
                })
                .unwrap_or_default(),
            RangeType::Range(..) => index
                .range_mut((Included(start), Included(end)))
                .filter_map(|(_k, v)| {
                    if v.is_null() {
                        None
                    } else {
                        let val = unsafe { Box::from_raw(*v) };
                        *v = ptr::null_mut();
                        Some(*val)
                    }
                })
                .collect(),
        }
    }

    fn insert_or_update(&self, key: K, value: V) -> Option<V> {
        let mut index = self.index.lock();
        index
            .insert(key, Box::into_raw(Box::new(value)))
            .map(|v| {
                if v.is_null() {
                    None
                } else {
                    Some(unsafe { *Box::from_raw(v) })
                }
            })
            .unwrap_or_default()
    }
}

unsafe impl<K: Ord, V> Sync for Index<K, V> {}

/// Type of `KeyRange`
enum RangeType<K: Ord> {
    /// `KeyRange` contains only one key
    OneKey(PhantomData<K>),
    /// `KeyRange` contains the keys in the range
    Range(PhantomData<K>),
}

impl<K: Ord> RangeType<K> {
    /// Get `RangeType` by given `key` and `range_end`
    fn get_range_type(key: &K, range_end: &K) -> Self {
        if key == range_end {
            RangeType::OneKey(PhantomData)
        } else {
            RangeType::Range(PhantomData)
        }
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
