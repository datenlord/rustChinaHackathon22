
use std::{
    io::Write,
    sync::atomic::{AtomicUsize, Ordering},
};

use crossbeam::epoch;
use epoch::{Atomic, Owned, Shared};

type NodePtr<T> = Atomic<Node<T>>;
struct Node<T> {
    pub item: Option<T>,
    pub next: NodePtr<T>,
}

impl<T> Node<T> {
    pub fn new_empty() -> Self {
        Self {
            item: None,
            next: Atomic::null(),
        }
    }

    pub fn new(data: T) -> Self {
        Self {
            item: Some(data),
            next: Atomic::null(),
        }
    }
}

pub struct Queue<T> {
    len: AtomicUsize,
    head: NodePtr<T>,
    tail: NodePtr<T>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        let head = Atomic::new(Node::new_empty());
        let tail = head.clone();
        Self {
            len: AtomicUsize::new(0),
            head,
            tail,
        }
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }

    pub fn is_empty(&self) -> bool {
        0 == self.len.load(Ordering::SeqCst)
    }

    pub fn push(&self, data: T) {
        let guard = epoch::pin();

        let new_node = Owned::new(Node::new(data)).into_shared(&guard);

        let mut tail;
        unsafe {
            let null = Shared::null();
            loop {
                tail = self.tail.load(Ordering::Acquire, &guard);
                let tail_next = &(*tail.as_raw()).next;
                if tail_next
                    .compare_exchange(null, new_node, Ordering::AcqRel, Ordering::Relaxed, &guard)
                    .is_ok()
                {
                    break;
                }
                let tail_next = tail_next.load(Ordering::Acquire, &guard);
                let _ = self.tail.compare_exchange(
                    tail,
                    tail_next,
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                    &guard,
                );
            }
        }
        let _ = self.tail.compare_exchange(
            tail,
            new_node,
            Ordering::Release,
            Ordering::Relaxed,
            &guard,
        );

        self.len.fetch_add(1, Ordering::SeqCst);
    }

    pub fn pop(&self) -> Option<T> {
        let mut data = None;
        if self.is_empty() {
            return data;
        }
        let guard = &epoch::pin();
        unsafe {
            loop {
                let head = self.head.load(Ordering::Acquire, guard);
                let mut next = (*head.as_raw()).next.load(Ordering::Acquire, guard);

                if next.is_null() {
                    return None;
                }

                if self
                    .head
                    .compare_exchange(head, next, Ordering::Release, Ordering::Relaxed, guard)
                    .is_ok()
                {
                    data = next.deref_mut().item.take();
                    guard.defer_destroy(head);
                    break;
                }
            }
        }
        self.len.fetch_sub(1, Ordering::SeqCst);
        data
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
        let guard = &epoch::pin();
        unsafe {
            let h = self.head.load_consume(guard);
            guard.defer_destroy(h);
        }
    }
}

impl<T> Queue<T> {
    // 用来打印元素
    pub fn walk(&self) {
        let _ = std::io::stdout().flush();
        let guard = epoch::pin();
        let mut start = self.head.load(Ordering::Acquire, &guard);

        let mut actual_len = 0;
        while !start.is_null() {
            unsafe {
                print!("-> {:?}", start.as_raw());
                let _ = std::io::stdout().flush();
                start = (*start.as_raw()).next.load(Ordering::Acquire, &guard);
            }
            actual_len += 1;
        }
        println!(" size:{} actual: {}", self.size(), actual_len - 1);
    }
}


#[cfg(test)]
mod lockfree_queue_test {
    use std::{
        sync::{
            atomic::{AtomicI32, Ordering},
            Arc, Barrier,
        },
        thread,
    };
    use crate::entry::Entry;
    use crate::lockfree_queue::{ Queue};


    #[test]
    fn test_single() {
        let q = Queue::new();
        let entry = Entry::new(1, 2);
        q.push(entry);
        q.walk();
    }
}