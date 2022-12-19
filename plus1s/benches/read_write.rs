use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ourtree::{IndexOperate, SkiplistIndex, XlineIndex};
use std::sync::{mpsc, Arc};
use std::thread;

fn read_write<I, F>(f: F)
where
    I: IndexOperate<i32, i32> + Sync,
    F: FnOnce() -> I,
{
    let max = black_box(100_000);
    let index = f();
    // run
    thread::scope(|scope| {
        let index = Arc::new(&index);
        let (wr_tx, wr_rx) = mpsc::channel();
        let (rd_tx, rd_rx) = mpsc::channel();
        let (dr_tx, dr_rx) = mpsc::channel();
        // write thread
        let w_index = index.clone();
        scope.spawn(move || {
            for i in 0..max {
                w_index.insert_or_update(i, i);
                wr_tx.send(i).unwrap();
            }
        });
        // delete thread
        let d_index = index.clone();
        scope.spawn(move || {
            for _ in 0..max {
                let i = rd_rx.recv().unwrap();
                let v = d_index.delete(&i, &i);
                assert_eq!(v, vec![i]);
                dr_tx.send(i).unwrap();
            }
        });
        // read after write thread
        let rw_index = index.clone();
        scope.spawn(move || {
            for _ in 0..max {
                let i = wr_rx.recv().unwrap();
                let v = rw_index.get(&i, &i);
                assert_eq!(v, vec![&i]);
                rd_tx.send(i).unwrap();
            }
        });
        // read after delete thread
        let dr_index = index;
        scope.spawn(move || {
            for _ in 0..max {
                let i = dr_rx.recv().unwrap();
                let v = dr_index.get(&i, &i);
                assert!(v.is_empty());
            }
        });
    })
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("read write xline", |b| {
        b.iter(|| read_write(|| XlineIndex::new()))
    });
    c.bench_function("read write crossbeam", |b| {
        b.iter(|| read_write(|| SkiplistIndex::new()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
