use std::fmt::Display;
use std::{cell::RefCell, rc::Rc};

type RefNode<K, V> = Rc<RefCell<Node<K, V>>>;

#[derive(Debug, Clone)]
pub struct Node<K: Ord + Clone + Copy, V: Clone + Copy> {
    key: K,
    value: V,
    down: Option<RefNode<K, V>>,
    next: Option<RefNode<K, V>>,
}

impl<K: Ord + Clone + Copy, V: Clone + Copy> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            down: None,
            next: None,
        }
    }

    pub fn new_ref_node(key: K, value: V) -> RefNode<K, V> {
        Rc::new(RefCell::new(Node::new(key, value)))
    }

    pub fn new_with_node(node: Node<K, V>) -> RefNode<K, V> {
        Rc::new(RefCell::new(node))
    }
}

#[derive(Debug, Clone)]
pub struct SkipList<K: Ord + Clone + Copy + Display, V: Clone + Copy> {
    head: RefCell<Option<RefNode<K, V>>>,
    max_level: usize,
}

impl<K: Ord + Clone + Copy + Display, V: Clone + Copy> SkipList<K, V> {
    pub fn new(max_level: usize) -> Self {
        SkipList {
            head: RefCell::new(None),
            max_level: max_level - 1,
        }
    }

    fn random_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }

    pub fn insert(&self, key: K, value: V) {
        let level = 1 + if !self.head.borrow().is_none() {
            self.random_level()
        } else {
            self.max_level
        };
        println!("level = {}", level);

        let node = Node::new(key, value);
        let mut current_level = self.max_level + 1;

        let h = { self.head.borrow().clone() };
        match h {
            Some(head) => {
                let mut current = head.clone();
                let mut up_node: Option<RefNode<K, V>> = None;
                loop {
                    let tmp = current.clone();
                    if current_level > level {
                        if tmp.borrow().key < key {
                            if let Some(next_node) = &tmp.borrow().next {
                                if next_node.borrow().key < key {
                                    current = Rc::clone(next_node);
                                } else {
                                    if let Some(down_node) = &tmp.borrow().down {
                                        current = Rc::clone(down_node);
                                        current_level -= 1;
                                    } else {
                                        return;
                                    }
                                }
                            } else {
                                if let Some(down_node) = &tmp.borrow().down {
                                    current = Rc::clone(down_node);
                                    current_level -= 1;
                                } else {
                                    return;
                                }
                            }
                        }
                    } else {
                        let current_key = { tmp.borrow().key };
                        if current_key < key {
                            let next_node = { &tmp.borrow().next.clone() };
                            if let Some(next_node) = next_node {
                                if next_node.borrow().key < key {
                                    current = Rc::clone(next_node);
                                    continue;
                                } else {
                                    let new_node = Node::new_with_node(node.clone());
                                    new_node.borrow_mut().next = Some(Rc::clone(next_node));
                                    current.borrow_mut().next = Some(new_node.clone());
                                    match up_node {
                                        Some(ref up) => {
                                            up.borrow_mut().down = Some(new_node.clone());
                                        }
                                        None => {
                                            up_node = Some(new_node.clone());
                                        }
                                    }
                                    if let Some(down_node) = &tmp.borrow().down {
                                        current = Rc::clone(down_node);
                                        current_level -= 1;
                                        continue;
                                    }
                                    return;
                                }
                            }
                            let new_node = Node::new_with_node(node.clone());
                            current.borrow_mut().next = Some(new_node.clone());
                            match up_node {
                                Some(ref up) => {
                                    up.borrow_mut().down = Some(new_node.clone());
                                }
                                None => {
                                    up_node = Some(new_node.clone());
                                }
                            }
                            if let Some(down_node) = &tmp.borrow().down {
                                current = Rc::clone(down_node);
                                current_level -= 1;
                            } else {
                                return;
                            }
                        }
                    }
                }
            }
            None => {
                let mut current = None;
                for i in 0..level {
                    if i == 0 {
                        let n = Node::new_with_node(node.clone());
                        *self.head.borrow_mut() = Some(n.clone());
                        current = Some(n.clone());
                        continue;
                    }
                    let new_node = Some(Node::new_with_node(node.clone()));
                    match current {
                        Some(ref c) => {
                            c.borrow_mut().down = new_node.clone();
                        }
                        None => {
                            return;
                        }
                    }
                    current = new_node.clone();
                }
            }
        }
    }

    pub fn insert_or_update(&self, key: K, value: V) {
        let level = 1 + if !self.head.borrow().is_none() {
            self.random_level()
        } else {
            self.max_level
        };

        let node = Node::new(key, value);
        let mut current_level = self.max_level + 1;

        let h = { self.head.borrow().clone() };
        match h {
            Some(head) => {
                let mut current = head.clone();
                let mut up_node: Option<RefNode<K, V>> = None;
                loop {
                    let mut tmp = current.clone();
                    let tmp_key = { tmp.borrow().key };
                    if tmp_key == key {
                        loop {
                            let new_tmp = tmp.clone();
                            {
                                tmp.borrow_mut().value = value;
                            }
                            if let Some(d) = &new_tmp.borrow().down {
                                tmp = Rc::clone(d);
                            } else {
                                return;
                            };
                        }
                    }
                    if current_level > level {
                        if tmp.borrow().key < key {
                            if let Some(next_node) = &tmp.borrow().next {
                                if next_node.borrow().key <= key {
                                    current = Rc::clone(next_node);
                                    continue;
                                } else {
                                    if let Some(down_node) = &tmp.borrow().down {
                                        current = Rc::clone(down_node);
                                        current_level -= 1;
                                    } else {
                                        return;
                                    }
                                }
                            } else {
                                if let Some(down_node) = &tmp.borrow().down {
                                    current = Rc::clone(down_node);
                                    current_level -= 1;
                                } else {
                                    return;
                                }
                            }
                        }
                    } else {
                        let current_key = { tmp.borrow().key };
                        if current_key < key {
                            let next_node = { &tmp.borrow().next.clone() };
                            if let Some(next_node) = next_node {
                                if next_node.borrow().key <= key {
                                    current = Rc::clone(next_node);
                                    continue;
                                } else {
                                    let new_node = Node::new_with_node(node.clone());
                                    new_node.borrow_mut().next = Some(Rc::clone(next_node));
                                    current.borrow_mut().next = Some(new_node.clone());
                                    match up_node {
                                        Some(ref up) => {
                                            up.borrow_mut().down = Some(new_node.clone());
                                        }
                                        None => {
                                            up_node = Some(new_node.clone());
                                        }
                                    }
                                    if let Some(down_node) = &tmp.borrow().down {
                                        current = Rc::clone(down_node);
                                        current_level -= 1;
                                        continue;
                                    }
                                    return;
                                }
                            }
                            let new_node = Node::new_with_node(node.clone());
                            current.borrow_mut().next = Some(new_node.clone());
                            match up_node {
                                Some(ref up) => {
                                    up.borrow_mut().down = Some(new_node.clone());
                                }
                                None => {
                                    up_node = Some(new_node.clone());
                                }
                            }
                            if let Some(down_node) = &tmp.borrow().down {
                                current = Rc::clone(down_node);
                                current_level -= 1;
                            } else {
                                return;
                            }
                        }
                    }
                }
            }
            None => {
                let mut current = None;
                for i in 0..level {
                    if i == 0 {
                        let n = Node::new_with_node(node.clone());
                        *self.head.borrow_mut() = Some(n.clone());
                        current = Some(n.clone());
                        continue;
                    }
                    let new_node = Some(Node::new_with_node(node.clone()));
                    match current {
                        Some(ref c) => {
                            c.borrow_mut().down = new_node.clone();
                        }
                        None => {
                            return;
                        }
                    }
                    current = new_node.clone();
                }
            }
        }
    }

    pub fn find(&self, key: K) -> Option<V> {
        let h = { self.head.borrow().clone() };
        match h {
            Some(ref head) => {
                let mut current = head.clone();
                loop {
                    let tmp = current.clone();
                    if tmp.borrow().key == key {
                        return Some(current.borrow().value);
                    }
                    if tmp.borrow().key < key {
                        if let Some(next_node) = &tmp.borrow().next {
                            if next_node.borrow().key <= key {
                                current = Rc::clone(next_node);
                            } else {
                                if let Some(down_node) = &tmp.borrow().down {
                                    current = Rc::clone(down_node);
                                } else {
                                    return None;
                                }
                            }
                        } else {
                            if let Some(down_node) = &tmp.borrow().down {
                                current = Rc::clone(down_node);
                            } else {
                                return None;
                            }
                        }
                    }
                }
            }
            None => None,
        }
    }

    // fn print_level_path(&self) {
    //     let h = { self.head.borrow().clone() };
    //     match h {
    //         Some(head) => {
    //             let mut current = head.clone();
    //             loop {
    //                 let tmp = current.clone();
    //                 // println!("key = {:?}, value = {}", tmp.borrow().key, tmp.borrow().value);
    //                 if let Some(value) = &tmp.borrow().down {
    //                     current = Rc::clone(value);
    //                 } else {
    //                     let mut n = tmp.clone();
    //                     loop {
    //                         let n_tmp = n.clone();
    //                         if let Some(value) = &n_tmp.borrow().next {
    //                             // println!(
    //                             //     "key = {}, value = {}",
    //                             //     value.borrow().key,
    //                             //     value.borrow().value
    //                             // );
    //                             n = Rc::clone(value);
    //                         } else {
    //                             return;
    //                         };
    //                     }
    //                     // return;
    //                 };
    //             }
    //         }
    //         None => {}
    //     }
    // }

    pub fn delete(&self, key: K) {
        let h = self.head.borrow().clone();
        match h {
            Some(head) => {
                let mut current = head.clone();
                loop {
                    let mut tmp = current.clone();
                    if tmp.borrow().key == key {
                        if let Some(d) = &tmp.borrow().next {
                            current = Rc::clone(d);
                        }
                        if let Some(d) = &tmp.borrow().down {
                            current = Rc::clone(d);
                        }
                    } else if tmp.borrow().key < key {
                        if let Some(next_node) = &tmp.borrow().next {
                            if next_node.borrow().key <= key {
                                current = Rc::clone(next_node);
                            } else {
                                if let Some(down_node) = &tmp.borrow().down {
                                    current = Rc::clone(down_node);
                                }
                            }
                        } else {
                            if let Some(down_node) = &tmp.borrow().down {
                                current = Rc::clone(down_node);
                            }
                        }
                    }
                }
            }
            None => {}
        }
    }

    pub fn get_range(&self, key: &K, range_end: &K) -> Vec<V> {
        let mut result = Vec::new();
        let h = { self.head.borrow().clone() };
        match h {
            Some(ref head) => {
                let mut current = head.clone();
                loop {
                    let mut tmp = current.clone();
                    if &tmp.borrow().key == key {
                        loop {
                            let new_tmp = tmp.clone();
                            if let Some(d) = &new_tmp.borrow().down {
                                tmp = Rc::clone(d);
                            } else {
                                break;
                            };
                        }
                        loop {
                            result.push(tmp.borrow().value);
                            let new_tmp = tmp.clone();
                            if let Some(next) = &new_tmp.borrow().next {
                                if &next.borrow().key <= range_end {
                                    tmp = Rc::clone(next);
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            };
                        }
                        return result;
                    }
                    if &tmp.borrow().key < key {
                        if let Some(next_node) = &tmp.borrow().next {
                            if &next_node.borrow().key <= key {
                                current = Rc::clone(next_node);
                            } else {
                                if let Some(down_node) = &tmp.borrow().down {
                                    current = Rc::clone(down_node);
                                } else {
                                    return result;
                                }
                            }
                        } else {
                            if let Some(down_node) = &tmp.borrow().down {
                                current = Rc::clone(down_node);
                            } else {
                                return result;
                            }
                        }
                    }
                }
            }
            None => {}
        }
        return result;
    }
}

#[cfg(test)]
mod tests {

    use super::SkipList;

    #[test]
    fn test() {
        let skiplist = SkipList::new(2);
        skiplist.insert_or_update(1, 1);
        skiplist.insert_or_update(3, 3);
        skiplist.insert_or_update(2, 2);
        skiplist.insert_or_update(4, 4);
        // skiplist.print_level_path();
        assert_eq!(Some(2), skiplist.find(2));
        assert_eq!(Some(3), skiplist.find(3));
        skiplist.insert_or_update(2, 5);
        assert_eq!(Some(5), skiplist.find(2));
        skiplist.insert_or_update(5, 5);
        assert_eq!(Some(5), skiplist.find(5));
        skiplist.insert_or_update(8, 8);
        assert_eq!(Some(8), skiplist.find(8));
        skiplist.insert_or_update(7, 7);
        assert_eq!(Some(7), skiplist.find(7));
        skiplist.insert_or_update(6, 6);
        assert_eq!(Some(6), skiplist.find(6));
        assert_eq!(vec![1, 5, 3, 4, 5, 6, 7], skiplist.get_range(&1, &7)); 
    }
}
