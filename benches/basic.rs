use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use hayami::SymbolTable;
use std::collections::HashMap;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut table = SymbolTable::new();
    let mut hash_table = HashMap::new();
    c.bench_function("rand level 0 insertion", |b| {
        b.iter(|| {
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
