use dashmap::DashMap;
use std::any::type_name;
use std::collections::BTreeMap;
use std::vec;
//#[warn(unreachable_code)]
fn get_usize<K>(any_name: K) -> usize {
    let addr = &any_name as *const K as usize;
    addr
}
fn usize_get<K>(addr: usize) -> *mut K {
    let pa = addr as *mut K;
    pa
}
struct Basedata {
    tree: DashMap<String, usize>,
    //pool: ThreadPool,
}
impl Basedata {
    fn new() -> Basedata {
        Basedata {
            tree: DashMap::new(),
            //pool: ThreadPool::new(10),
            //listener: mpsc::channel(),
        }
    }
    //fn new_chanel<K, V>() {}
    fn new_tree<K: Ord, V>(&self) -> Option<bool> {
        let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        let root_temp = self.contains::<K, V>(&root_name);
        if root_temp.is_some() {
            return Some(false);
        } else {
            let new_tree = BTreeMap::<K, V>::new();
            self.tree.insert(root_name, get_usize(new_tree));

            return Some(true);
        }
        //None
    }
    fn contains<K: Ord + Copy, V: Copy>(&self, root_name: &String) -> Option<&mut BTreeMap<K, V>> {
        //let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        if self.tree.contains_key(root_name) {
            let root_addr = *self.tree.get(root_name).unwrap();
            unsafe {
                let root = usize_get::<BTreeMap<K, V>>(root_addr).as_mut().unwrap();
                return Some(root);
            }
        }
        None
    }
    fn insert<K: Ord + Copy, V: Copy>(&self, tree: &mut BTreeMap<K, V>) {
        let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        let root_temp = self.contains::<K, V>(&root_name);
        if root_temp.is_some() {
            let root = root_temp.unwrap();
            for (&key, val) in tree {
                let add_key = key.clone();
                let add_val = val.clone();
                root.insert(add_key, add_val);
            }
        } else {
            self.tree.insert(root_name, get_usize(tree));
        }
    }
    fn get<K: Ord + Copy, V: Copy>(&self, key: &K, range_end: &K) -> Vec<&V> {
        let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        let root_temp = self.contains::<K, V>(&root_name);
        if root_temp.is_some() {
            let root = root_temp.unwrap();
            let mut ret_vec = Vec::new();
            for (_, V) in root.range(key..range_end) {
                ret_vec.push(V);
            }
        }
        vec![]
    }
    fn delete<K: Ord + Copy, V: Copy>(&self, key: &K, range_end: &K) -> Vec<V> {
        let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        let root_temp = self.contains::<K, V>(&root_name);
        if root_temp.is_some() {
            let root = root_temp.unwrap();
            let mut ret_vec = Vec::new();
            for (_, &val) in root.range(key..range_end) {
                ret_vec.push(val);
            }
            root.retain(|&k, _| k < *key && k > *range_end);
            return ret_vec;
        }
        vec![]
    }
    fn insert_or_update<K: Ord + Copy, V: Copy>(&self, key: K, value: V) -> Option<V> {
        let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        let root_temp = self.contains::<K, V>(&root_name);
        if root_temp.is_some() {
            let root = root_temp.unwrap();
            return root.insert(key, value);
        } else {
            let flag_task = self.new_tree::<K, V>();
            if flag_task == Some(true) {
                let root_temp = self.contains::<K, V>(&root_name);
                let root = root_temp.unwrap();
                return root.insert(key, value);
            }
        }
        None
    }
    // fn thread_task(&self) {
    //     //let rec = self.listener.1.recv().unwrap();
    // }
}
trait IndexOperate<K: Ord, V> {
    fn get(&self, key: &K, range_end: &K) -> Vec<&V>;
    fn delete(&self, key: &K, range_end: &K) -> Vec<V>;
    fn insert_or_update(&self, key: K, value: V) -> Option<V>;
}
impl<K: Ord, V> IndexOperate<K, V> for Basedata {
    fn get(&self, key: &K, range_end: &K) -> Vec<&V> {
        self.get(key, range_end)
    }

    fn delete(&self, key: &K, range_end: &K) -> Vec<V> {
        self.delete(key, range_end)
    }

    fn insert_or_update(&self, key: K, value: V) -> Option<V> {
        self.insert_or_update(key, value)
    }
}
