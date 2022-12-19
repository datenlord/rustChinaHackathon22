use guduyaogun::Index;
use guduyaogun::IndexOperate;
use guduyaogun::SimpleIndex;

use num_cpus;

#[test]
fn simple_insert() {
    let index: SimpleIndex<u32, String> = SimpleIndex::new();

    let zero = index.get(&1, &1);
    assert_eq!(zero.len(), 0);

    let _ = index.insert_or_update(1, "value1".to_string());
    let _ = index.insert_or_update(2, "value2".to_string());
    let _ = index.insert_or_update(3, "value3".to_string());
    let three = index.get(&1, &3);
    assert_eq!(three.len(), 3);
    let two = index.get(&2, &2);
    assert_eq!(two.len(), 1);
    assert_eq!(two[0], "value2");
}

#[test]
fn simple_delete() {
    let index: SimpleIndex<u32, String> = SimpleIndex::new();

    let _ = index.insert_or_update(1, "value1".to_string());
    let _ = index.insert_or_update(2, "value2".to_string());
    let _ = index.insert_or_update(3, "value3".to_string());
    let three = index.get(&1, &3);
    assert_eq!(three.len(), 3);

    let _ = index.delete(&2, &3);
    let one = index.get(&1, &3);
    assert_eq!(one.len(), 1);
}

#[test]
fn simple_multi_threads() {
    let index: SimpleIndex<u32, String> = SimpleIndex::new();

    let threadpool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .unwrap();

    threadpool.scope(|s| {
        for i in 0..100000 {
            let index = index.clone();
            s.spawn(move |_| {
                let _ = index.insert_or_update(i as u32, format!("value{}", i));
            })
        }
    });
    assert_eq!(index.len(), 100000);
}
