use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ourtree::{IndexOperate, SkiplistIndex, XlineIndex};
use std::sync::Arc;
use std::thread;

fn read_only<I, F>(f: F)
where
    I: IndexOperate<i32, i32> + Sync,
    F: FnOnce() -> I,
{
    let max = black_box(1_000);
    let times = black_box(10);
    let threads = black_box(16);
    let index = f();
    // prepare
    for i in 0..max {
        index.insert_or_update(i, i);
    }
    // run
    thread::scope(|scope| {
        let index = Arc::new(&index);
        for _ in 0..threads {
            let index = index.clone();
            scope.spawn(move || {
                for _ in 0..times {
                    for i in 0..max {
                        let v = index.get(&i, &i);
                        assert_eq!(v, vec![&i]);
                    }
                }
            });
        }
    })
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("read only xline", |b| {
        b.iter(|| read_only(|| XlineIndex::new()))
    });
    c.bench_function("read only crossbeam", |b| {
        b.iter(|| read_only(|| SkiplistIndex::new()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
