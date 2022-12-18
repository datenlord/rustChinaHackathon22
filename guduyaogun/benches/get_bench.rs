use criterion::*;

use num_cpus;

use guduyaogun::CSIndex;
use guduyaogun::Index;
use guduyaogun::IndexOperate;
use guduyaogun::SimpleIndex;
use guduyaogun::SkipMapIndex;

pub fn simple_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_index_bench");
    group.sample_size(10);
    group.bench_function("get_multi_threads", |b| {
        b.iter_batched(
            || {
                let index: SimpleIndex<u32, String> = SimpleIndex::new();
                for i in 0..5_000_000 {
                    let _ = index.insert_or_update(i as u32, format!("value{}", i));
                }
                assert_eq!(index.len(), 5_000_000);
                let threadpool = rayon::ThreadPoolBuilder::new()
                    .num_threads(num_cpus::get())
                    .build()
                    .unwrap();
                (index, threadpool)
            },
            |(index, threadpool)| {
                threadpool.scope(|s| {
                    for i in 0..500 {
                        let index = index.clone();
                        let start = i * 10_000;
                        let end = start + 9_999;
                        s.spawn(move |_| {
                            index.get(&start, &end);
                        })
                    }
                });
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("get_single_thread", |b| {
        b.iter_batched(
            || {
                let index: SimpleIndex<u32, String> = SimpleIndex::new();
                for i in 0..5_000_000 {
                    let _ = index.insert_or_update(i as u32, format!("value{}", i));
                }
                assert_eq!(index.len(), 5_000_000);
                index
            },
            |index| {
                for i in 0..500 {
                    let start = i * 10_000;
                    let end = start + 9_999;
                    index.get(&start, &end);
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

pub fn cs_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("cs_index_bench");
    group.sample_size(10);
    group.bench_function("get_multi_threads", |b| {
        b.iter_batched(
            || {
                let index: CSIndex<u32, String> = CSIndex::new();
                for i in 0..5_000_000 {
                    let _ = index.insert_or_update(i as u32, format!("value{}", i));
                }
                assert_eq!(index.len(), 5_000_000);
                let threadpool = rayon::ThreadPoolBuilder::new()
                    .num_threads(num_cpus::get())
                    .build()
                    .unwrap();
                (index, threadpool)
            },
            |(index, threadpool)| {
                threadpool.scope(|s| {
                    for i in 0..500 {
                        let index = index.clone();
                        let start = i * 10_000;
                        let end = start + 9_999;
                        s.spawn(move |_| {
                            index.get(&start, &end);
                        })
                    }
                });
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("get_single_thread", |b| {
        b.iter_batched(
            || {
                let index: CSIndex<u32, String> = CSIndex::new();
                for i in 0..5_000_000 {
                    let _ = index.insert_or_update(i as u32, format!("value{}", i));
                }
                assert_eq!(index.len(), 5_000_000);
                index
            },
            |index| {
                for i in 0..500 {
                    let start = i * 10_000;
                    let end = start + 9_999;
                    index.get(&start, &end);
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

pub fn skipmap_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("skipmap_index_bench");
    group.sample_size(10);
    group.bench_function("get_multi_threads", |b| {
        b.iter_batched(
            || {
                let index: SkipMapIndex<u32, String> = SkipMapIndex::new();
                for i in 0..5_000_000 {
                    let _ = index.insert_or_update(i as u32, format!("value{}", i));
                }
                let threadpool = rayon::ThreadPoolBuilder::new()
                    .num_threads(num_cpus::get())
                    .build()
                    .unwrap();
                (index, threadpool)
            },
            |(index, threadpool)| {
                threadpool.scope(|s| {
                    for i in 0..500 {
                        let index = index.clone();
                        let start = i * 10_000;
                        let end = start + 9_999;
                        s.spawn(move |_| {
                            index.get(&start, &end);
                        })
                    }
                });
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("get_single_thread", |b| {
        b.iter_batched(
            || {
                let index: SkipMapIndex<u32, String> = SkipMapIndex::new();
                for i in 0..5_000_000 {
                    let _ = index.insert_or_update(i as u32, format!("value{}", i));
                }
                index
            },
            |index| {
                for i in 0..500 {
                    let start = i * 10_000;
                    let end = start + 9_999;
                    index.get(&start, &end);
                }
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}
