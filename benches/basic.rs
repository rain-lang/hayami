use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use hayami::SymbolTable;
use std::collections::HashMap;
use ahash::RandomState;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut table = SymbolTable::new();
    let mut hash_table = HashMap::<usize, usize, RandomState>::default();
    c.bench_function("SymbolTable: level 0 insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| {
            table.insert(key, value)
        })
    });
    c.bench_function("HashMap: insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| {
            hash_table.insert(key, value)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
