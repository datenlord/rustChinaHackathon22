use criterion::{criterion_group, criterion_main, Criterion};

mod del_bench;
mod get_bench;
mod insert_bench;

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets =
    crate::get_bench::simple_index,
    crate::get_bench::cs_index,
    crate::get_bench::skipmap_index,
    crate::del_bench::simple_index,
    crate::del_bench::cs_index,
    crate::del_bench::skipmap_index,
    crate::insert_bench::simple_index,
    crate::insert_bench::cs_index,
    crate::insert_bench::skipmap_index,
);

criterion_main!(benches);
