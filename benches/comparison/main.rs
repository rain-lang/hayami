/*!
Compare `SymbolTable` performance to other `HashMap`s 
*/

use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use ahash::RandomState;
use fxhash::FxHashMap;
use indexmap::IndexMap;
use hayami::SymbolTable;
use im;
use im_rc;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut im_table = im::HashMap::<usize, usize, RandomState>::default();
    c.bench_function("im::HashMap: insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| {
            im_table.insert(key, value)
        })
    });
    std::mem::drop(im_table);
    let mut im_rc_table = im_rc::HashMap::<usize, usize, RandomState>::default();
    c.bench_function("im_rc::HashMap: insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| {
            im_rc_table.insert(key, value)
        })
    });
    std::mem::drop(im_rc_table);
    let mut table = SymbolTable::new();
    c.bench_function("SymbolTable: level 0 insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| {
            table.insert(key, value)
        })
    });
    std::mem::drop(table);
    let mut index_table = IndexMap::<usize, usize, RandomState>::default();
    c.bench_function("IndexMap: insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| {
            index_table.insert(key, value)
        })
    });
    std::mem::drop(index_table);
    let mut hash_table = HashMap::<usize, usize, RandomState>::default();
    c.bench_function("HashMap: insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| {
            hash_table.insert(key, value)
        })
    });
    std::mem::drop(hash_table);
    let mut fxhash_table = FxHashMap::<usize, usize>::default();
    c.bench_function("FxHashMap: insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| {
            fxhash_table.insert(key, value)
        })
    });
    std::mem::drop(fxhash_table);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
