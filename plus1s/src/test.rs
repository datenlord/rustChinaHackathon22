use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use crate::IndexOperate;

pub(crate) fn basic<I, F>(f: F)
where
    I: IndexOperate<i32, i32>,
    F: FnOnce() -> I,
{
    let index = f();
    // empty read
    assert!(index.get(&1, &4).is_empty());
    // non-empty read
    assert_eq!(index.insert_or_update(2, 2), None);
    assert!(index.get(&1, &1).is_empty());
    assert_eq!(index.get(&1, &2), vec![&2]);
    assert_eq!(index.get(&1, &4), vec![&2]);
    assert_eq!(index.insert_or_update(3, 33), None);
    assert_eq!(index.get(&1, &4), vec![&2, &33]);
    assert_eq!(index.insert_or_update(4, 444), None);
    assert_eq!(index.get(&1, &4), vec![&2, &33, &444]);
    // replace
    assert_eq!(index.insert_or_update(4, 555), Some(444));
    assert_eq!(index.get(&1, &4), vec![&2, &33, &555]);
    // delete
    assert_eq!(index.delete(&3, &4), vec![33, 555]);
    assert_eq!(index.get(&1, &4), vec![&2]);
}

pub(crate) fn multithread<I, F>(f: F)
where
    I: IndexOperate<i32, i32> + Sync,
    F: FnOnce() -> I,
{
    let (tx, rx) = mpsc::channel();
    let index = f();
    let index = Arc::new(&index);
    let write_idnex = index.clone();
    thread::scope(|scope| {
        scope.spawn(move || {
            for i in 0..100 {
                write_idnex.insert_or_update(i, i);
                tx.send(i).unwrap();
            }
        });
        for _ in 0..100 {
            let written_key = rx.recv().unwrap();
            assert_eq!(index.get(&written_key, &written_key), vec![&written_key]);
        }
    });
}
