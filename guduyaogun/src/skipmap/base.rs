extern crate rand;
use core::sync::atomic::Ordering::SeqCst;
use std::alloc::{GlobalAlloc, Layout, System};
use std::marker::PhantomData;
use std::mem;
use std::sync::atomic::AtomicUsize;

/// In this Hackthon, we assumes no conflicts, so the unsafe operations here seem to be safe.

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: Allocator = Allocator;

struct Node<K, V> {
    key: K,
    value: V,
    lanes: [AtomicUsize; 25],
}
impl<K: Ord + Clone, V> Node<K, V> {
    pub fn new_addr(key: K, val: V) -> usize {
        let node_layout = Layout::new::<Node<K, V>>();
        let alloc: *mut u8 = unsafe { Allocator.alloc(node_layout) };
        let node = alloc as *mut Node<K, V>;
        unsafe {
            std::ptr::write(
                node,
                Node {
                    key,
                    value: val,
                    lanes: mem::zeroed(),
                },
            );
        }
        node as usize
    }

    fn store(&self, addr: usize, height: usize) {
        let atomic_ref = unsafe { self.lanes.get_unchecked(height) };
        atomic_ref.store(addr, SeqCst);
    }
}

impl<K, V> Node<K, V> {
    fn get_at(&self, height: usize) -> Option<AtomicPtrs<K, V>> {
        let atomic_ref = unsafe { self.lanes.get_unchecked(height) };
        let addr = atomic_ref.load(SeqCst);
        if addr == 0 {
            return None;
        }
        return Some(AtomicPtrs {
            loaded_addr: addr,
            addr: atomic_ref,
            node: unsafe { mem::transmute(addr) },
        });
    }
    fn dealloc(&self) {
        let node_layout = Layout::new::<Node<K, V>>();
        unsafe {
            Allocator.dealloc(
                std::ptr::NonNull::new(self as *const Node<K, V> as *mut Node<K, V> as *mut u8)
                    .unwrap()
                    .as_ptr(),
                node_layout,
            );
        }
    }
    fn value(&self) -> &V {
        &self.value
    }
}

struct AtomicPtrs<'a, K: 'a, V: 'a> {
    loaded_addr: usize,
    addr: &'a AtomicUsize,
    node: &'a Node<K, V>,
}

impl<'a, K: 'a, V: 'a> AtomicPtrs<'a, K, V> {
    #[allow(dead_code)]
    fn compare_exchange(&self, addr: usize) -> bool {
        if self.loaded_addr
            == self
                .addr
                .compare_exchange(self.loaded_addr, addr, SeqCst, SeqCst)
                .unwrap()
        {
            return true;
        }
        return false;
    }
}

pub struct SkipMap<K, V> {
    head: Node<K, V>,
    height: AtomicUsize,
    _key_type: PhantomData<K>,
    _value_type: PhantomData<V>,
}

pub fn random_height() -> usize {
    const MASK: u32 = 1 << 23;
    1 + (rand::random::<u32>() | MASK).trailing_zeros() as usize
}

impl<K, V> SkipMap<K, V> {
    fn get_at(&self, height: usize) -> Option<AtomicPtrs<K, V>> {
        let atomic_ref = unsafe { self.head.lanes.get_unchecked(height) };
        let addr = atomic_ref.load(SeqCst);
        if addr == 0 {
            return None;
        }
        return Some(AtomicPtrs {
            loaded_addr: addr,
            addr: atomic_ref,
            node: unsafe { mem::transmute(addr) },
        });
    }
}

impl<K: Default, V: Default> SkipMap<K, V> {
    pub fn new() -> SkipMap<K, V> {
        SkipMap {
            head: Node {
                key: Default::default(),
                value: Default::default(),
                lanes: unsafe { mem::zeroed() },
            },
            height: AtomicUsize::new(0),
            _value_type: Default::default(),
            _key_type: Default::default(),
        }
    }
}

impl<K: Ord + Clone, V: Clone> SkipMap<K, V> {
    #[allow(dead_code)]
    fn store(&self, addr: usize, height: usize) {
        let atomic_ref = unsafe { self.head.lanes.get_unchecked(height) };
        atomic_ref.store(addr, SeqCst);
    }

    fn find(&self, key: &K) -> Option<&Node<K, V>> {
        let p = self.lower_bound_before(key);
        match p.get_at(0) {
            None => None,
            Some(ptr) => {
                if *key == ptr.node.key {
                    return Some(ptr.node);
                }
                None
            }
        }
    }

    /// get exactly from skipmap
    #[allow(dead_code)]
    pub fn get(&self, key: &K) -> Option<&V> {
        match self.find(key) {
            Some(node) => {
                return Some(&node.value);
            }
            None => None,
        }
    }

    /// Inserts data into the SkipMap
    pub fn insert_or_update(&self, key: K, value: V) -> Option<V> {
        // check for update
        if let Some(_node) = self.find(&key) {
            let v = _node.value.clone();
            let const_ptr = _node as *const Node<K, V>;
            let mut_ptr = const_ptr as *mut Node<K, V>;
            unsafe {
                (*mut_ptr).value = value;
            };
            return Some(v);
        }

        // Insert from base
        let height = random_height();
        let current_height = self.height.load(SeqCst);

        if height > current_height {
            let _ = self
                .height
                .compare_exchange(current_height, height, SeqCst, SeqCst);
        }
        let x_addr = Node::new_addr(key.clone(), value);

        // will be used in setting the next addresss.
        let x_node: &Node<K, V> = unsafe { mem::transmute(x_addr) };

        let before_tower = self.lower_bound_before_all_levels(&key);
        for i in 0..=height {
            if let Some(next) = before_tower[i].get_at(i) {
                x_node.store(next.loaded_addr, i);
            }
            before_tower[i].store(x_addr, i);
        }
        None
    }

    /// return the node before the lower_bound
    fn lower_bound_before(&self, key: &K) -> &Node<K, V> {
        let mut p = &self.head;
        let mut height = self.height.load(SeqCst);
        let mut next: AtomicPtrs<K, V>;
        loop {
            next = match p.get_at(height) {
                Some(ptr) => ptr,
                None => {
                    if height > 0 {
                        height = height - 1;
                        continue;
                    }
                    break;
                }
            };
            if next.node.key < *key {
                p = next.node;
            } else {
                if height > 0 {
                    height = height - 1;
                    continue;
                }
                break;
            }
        }
        p
    }

    /// return the node before the lower_bound in all levels
    fn lower_bound_before_all_levels(&self, key: &K) -> Vec<&Node<K, V>> {
        let mut p = &self.head;
        let mut height = self.height.load(SeqCst);
        let mut next: AtomicPtrs<K, V>;
        let mut tower: Vec<&Node<K, V>> = Vec::new();
        tower.resize(height + 1, p);
        loop {
            next = match p.get_at(height) {
                Some(ptr) => ptr,
                None => {
                    tower[height] = p;
                    if height > 0 {
                        height = height - 1;
                        continue;
                    }
                    break;
                }
            };
            if next.node.key < *key {
                p = next.node;
            } else {
                tower[height] = p;
                if height > 0 {
                    height = height - 1;
                    continue;
                }
                break;
            }
        }
        tower
    }

    pub fn get_range(&self, key: &K, range_end: &K) -> Vec<&V> {
        let mut p = self.lower_bound_before(key);
        p = match p.get_at(0) {
            None => return Vec::new(),
            Some(ptr) => ptr.node,
        };
        let mut rst: Vec<&V> = Vec::new();
        while p.key <= *range_end {
            let value_addr = unsafe { &*(p.value() as *const V) };
            rst.push(value_addr);
            p = match p.get_at(0) {
                None => break,
                Some(ptr) => ptr.node,
            };
        }
        rst
    }

    pub fn del_range(&self, key: &K, range_end: &K) -> Vec<V> {
        let tower = self.lower_bound_before_all_levels(key);
        let head = tower[0];
        let mut rst: Vec<V> = Vec::new();
        while let Some(next) = head.get_at(0) {
            if next.node.key > *range_end {
                break;
            }
            // todo(wujs): may be wrong
            // for i in 0..self.height.load(SeqCst) {
            //     head.lanes[i].store(next.node.lanes[i].load(SeqCst), SeqCst);
            // }
            for i in 0..tower.len() {
                if let Some(ptr) = tower[i].get_at(i) {
                    if ptr.node.key == next.node.key {
                        tower[i].store(next.node.lanes[i].load(SeqCst), i);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            rst.push(next.node.value().clone());
            next.node.dealloc();
        }
        rst
    }
}

impl<K, V> Drop for SkipMap<K, V> {
    fn drop(&mut self) {
        //drop one bye one in the bottom
        let mut bottom = match self.get_at(0) {
            Some(node) => node,
            None => return,
        };
        loop {
            match bottom.node.get_at(0) {
                Some(ptrs) => {
                    bottom.node.dealloc();
                    bottom = ptrs;
                }
                None => {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SkipMap;

    #[test]
    fn test_insert_or_update() {
        let skip_map: SkipMap<String, String> = SkipMap::new();
        skip_map.insert_or_update(String::from("a"), String::from("1"));
        skip_map.insert_or_update(String::from("b"), String::from("2"));
        skip_map.insert_or_update(String::from("d"), String::from("4"));
        skip_map.insert_or_update(String::from("g"), String::from("7"));

        skip_map.insert_or_update(String::from("h"), String::from("8"));
        skip_map.insert_or_update(String::from("i"), String::from("9"));
        skip_map.insert_or_update(String::from("e"), String::from("6"));
        skip_map.insert_or_update(String::from("f"), String::from("5"));

        let old = skip_map.insert_or_update(String::from("a"), String::from("2"));
        assert_eq!(old.unwrap(), "1");
        assert_eq!(skip_map.get(&String::from("a")).expect("expected 2"), "2");
        assert_eq!(skip_map.get(&String::from("g")).expect("expected 7"), "7");
    }

    #[test]
    fn test_lower_bound() {
        let skip_map: SkipMap<String, String> = SkipMap::new();
        skip_map.insert_or_update(String::from("a"), String::from("1"));
        skip_map.insert_or_update(String::from("b"), String::from("2"));
        skip_map.insert_or_update(String::from("d"), String::from("4"));
        skip_map.insert_or_update(String::from("g"), String::from("7"));

        skip_map.insert_or_update(String::from("h"), String::from("8"));
        skip_map.insert_or_update(String::from("i"), String::from("9"));
        skip_map.insert_or_update(String::from("e"), String::from("6"));
        skip_map.insert_or_update(String::from("f"), String::from("5"));

        let node = skip_map.lower_bound_before(&String::from("g"));
        assert_eq!(node.key, "f");

        let node = skip_map.lower_bound_before(&String::from("j"));
        assert_eq!(node.key, "i");

        let node = skip_map.lower_bound_before(&String::from("a"));
        assert_eq!(node.key, "");
    }

    #[test]
    fn test_lower_bound_all_levels() {
        let skip_map: SkipMap<String, String> = SkipMap::new();
        skip_map.insert_or_update(String::from("a"), String::from("1"));
        skip_map.insert_or_update(String::from("b"), String::from("2"));
        skip_map.insert_or_update(String::from("d"), String::from("4"));
        skip_map.insert_or_update(String::from("g"), String::from("7"));

        skip_map.insert_or_update(String::from("h"), String::from("8"));
        skip_map.insert_or_update(String::from("i"), String::from("9"));
        skip_map.insert_or_update(String::from("e"), String::from("6"));
        skip_map.insert_or_update(String::from("f"), String::from("5"));

        // let vs = skip_map.lower_bound_before_all_levels(&String::from("k"));
        // for v in vs {
        //     println!(">> {}", v.key);
        // }
    }

    #[test]
    fn test_range_get() {
        let skip_map: SkipMap<u32, String> = SkipMap::new();
        for i in 0..100 {
            skip_map.insert_or_update(i as u32, format!("value{}", i));
        }

        let vs = skip_map.get_range(&0, &49);
        assert_eq!(vs.len(), 50);

        let vs = skip_map.get_range(&0, &10000);
        assert_eq!(vs.len(), 100);
    }

    #[test]
    fn test_range_delete() {
        let skip_map: SkipMap<u32, String> = SkipMap::new();
        for i in 0..100 {
            skip_map.insert_or_update(i as u32, format!("value{}", i));
        }

        let vs = skip_map.del_range(&30, &39);
        assert_eq!(vs.len(), 10);

        let vs = skip_map.get_range(&0, &10000);
        assert_eq!(vs.len(), 90);
    }
}
