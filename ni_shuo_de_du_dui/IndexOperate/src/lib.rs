use dashmap::DashMap;
use std::any::type_name;
use std::collections::BTreeMap;
use std::ops::Bound::Included;
use std::vec;
//#[warn(unreachable_code)]
struct Basedata {
    tree: DashMap<String, usize>,
    //pool: ThreadPool,
}
impl Basedata {
    pub fn new() -> Basedata {
        let data = DashMap::new();
        Basedata {
            tree: data,
            //pool: ThreadPool::new(10),
            //listener: mpsc::channel(),
        }
    }
    fn get_usize<'a, K>(any_name: &K) -> usize {
        let addr = any_name as *const K as usize;
        addr
    }
    fn usize_get<'a, K>(addr: usize) -> *mut K {
        let pa = addr as *mut K;
        pa
    }
    //fn new_chanel<K, V>() {}
    fn new_tree<'a, K: Ord, V>(&self) -> BTreeMap<K, V> {
        let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        let new_tree = BTreeMap::<K, V>::new();

        //let root_n = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        self.tree.insert(root_name, Self::get_usize(&new_tree));
        new_tree
    }
    fn contains<K: Ord, V>(&self, root_name: &String) -> Option<&mut BTreeMap<K, V>> {
        //let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        if self.tree.contains_key(root_name) {
            let root_addr = *self.tree.get(root_name).unwrap();
            unsafe {
                let root = Self::usize_get::<BTreeMap<K, V>>(root_addr)
                    .as_mut()
                    .unwrap();
                return Some(root);
            }
        }
        None
    }
    // fn insert<K: Ord, V>(&self, tree: &mut BTreeMap<K, V>) {
    //     let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
    //     let root_temp = self.contains::<K, V>(&root_name);
    //     if root_temp.is_some() {
    //         let root = root_temp.unwrap();
    //         for (&key, val) in tree {
    //             root.insert(&add_key, add_val);
    //         }
    //     } else {
    //         self.tree.insert(root_name, get_usize(tree));
    //     }
    // }
    // fn thread_task(&self) {
    //     //let rec = self.listener.1.recv().unwrap();
    // }
}
trait IndexOperate<K: Ord, V: Copy> {
    fn get(&self, key: &K, range_end: &K) -> Vec<&V>;
    fn delete(&self, key: &K, range_end: &K) -> Vec<V>;
    fn insert_or_update(&self, key: K, value: V) -> Option<V>;
}
impl<'a, K: Ord + 'static, V: Copy> IndexOperate<K, V> for Basedata {
    fn get(&self, key: &K, range_end: &K) -> Vec<&V> {
        let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        let root_temp = self.contains::<K, V>(&root_name);
        if root_temp.is_some() {
            let root = root_temp.unwrap();
            let mut ret_vec = Vec::new();
            for (_, v) in root.range(key..range_end) {
                ret_vec.push(v);
            }
            return ret_vec;
        }
        vec![]
    }

    fn delete(&self, key: &K, range_end: &K) -> Vec<V> {
        let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        let root_temp = self.contains::<K, V>(&root_name);
        if root_temp.is_some() {
            let root = root_temp.unwrap();
            let mut ret_vec = Vec::new();
            for (_, val) in root.range((Included(key), Included(range_end))) {
                let temp = val.clone();
                ret_vec.push(temp);
            }
            root.retain(|k, _| k < key && k > range_end);
            return ret_vec;
        }
        vec![]
    }

    fn insert_or_update(&self, key: K, value: V) -> Option<V> {
        let root_name = format!("{},{}", type_name::<Option<K>>(), type_name::<Option<V>>());
        let root_temp = self.contains::<K, V>(&root_name);
        if root_temp.is_some() {
            let root = root_temp.unwrap();
            let ret = root.insert(key, value);
            //println!("{}", root.len());
            return ret;
        } else {
            self.new_tree::<K, V>();
            let root_addr = *self.tree.get(&root_name).unwrap();
            unsafe {
                let root = Self::usize_get::<BTreeMap<K, V>>(root_addr)
                    .as_mut()
                    .unwrap();
                let ret = root.insert(key, value);
                //println!("{}", root.len());
                return ret;
            }
            //println!("{}", root.len());
            //println!("{}", root.len());
        }
    }
}
