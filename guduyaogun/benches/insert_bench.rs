use criterion::*;

use guduyaogun::CSIndex;
use guduyaogun::Index;
use guduyaogun::IndexOperate;
use guduyaogun::SimpleIndex;
use guduyaogun::SkipMapIndex;

pub fn simple_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_index_bench");
    group.bench_function("insert_single_thread", |b| {
        b.iter_batched(
            || {
                let index: SimpleIndex<u32, String> = SimpleIndex::new();
                index
            },
            |index| {
                for i in 0..5_000 {
                    let _ = index.insert_or_update(i as u32, format!("value{}", i));
                }
                assert_eq!(index.len(), 5_000);
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

pub fn cs_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("cs_index_bench");
    group.bench_function("insert_single_thread", |b| {
        b.iter_batched(
            || {
                let index: CSIndex<u32, String> = CSIndex::new();
                index
            },
            |index| {
                for i in 0..5_000 {
                    let _ = index.insert_or_update(i as u32, format!("value{}", i));
                }
                assert_eq!(index.len(), 5_000);
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

pub fn skipmap_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("skipmap_index_bench");
    group.bench_function("insert_single_thread", |b| {
        b.iter_batched(
            || {
                let index: SkipMapIndex<u32, String> = SkipMapIndex::new();
                index
            },
            |index| {
                for i in 0..5_000 {
                    let _ = index.insert_or_update(i as u32, format!("value{}", i));
                }
                assert_eq!(index.len(), 5_000);
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}
