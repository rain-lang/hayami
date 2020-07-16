/*!
Compare `SymbolTable` performance to other `HashMap`s
*/

use ahash::RandomState;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fxhash::FxHashMap;
use im;
use im_rc;
use indexmap::IndexMap;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub mod old;
use old::SymbolTable as OldSymbolTable;

pub fn insertion_benchmarks(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut im_table = im::HashMap::<usize, usize, RandomState>::default();
    c.bench_function("im::HashMap: insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| im_table.insert(key, value))
    });
    std::mem::drop(im_table);
    let mut im_rc_table = im_rc::HashMap::<usize, usize, RandomState>::default();
    c.bench_function("im_rc::HashMap: insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| im_rc_table.insert(key, value))
    });
    std::mem::drop(im_rc_table);
    let mut table = OldSymbolTable::<usize, usize>::new();
    c.bench_function("Old SymbolTable: level 0 insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| table.insert(key, value))
    });
    std::mem::drop(table);
    let mut index_table = IndexMap::<usize, usize, RandomState>::default();
    c.bench_function("IndexMap: insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| index_table.insert(key, value))
    });
    std::mem::drop(index_table);
    let mut hash_table = HashMap::<usize, usize, RandomState>::default();
    c.bench_function("HashMap: insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| hash_table.insert(key, value))
    });
    std::mem::drop(hash_table);
    let mut fxhash_table = FxHashMap::<usize, usize>::default();
    c.bench_function("FxHashMap: insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| fxhash_table.insert(key, value))
    });
    std::mem::drop(fxhash_table);
}

pub fn layer_benchmarks(c: &mut Criterion) {
    let layer1: Vec<(usize, usize)> = (0..10000).map(|u| (u, 2 * u)).collect();
    let layer2: Vec<(usize, usize)> = (250..750).map(|u| (u, 3 * u)).collect();
    let layer3: Vec<(usize, usize)> = (500..7000).map(|u| (u, 2 * u)).collect();
    let layer2_2: Vec<(usize, usize)> = (100..9000).map(|u| (u, u)).collect();
    let layer3_2: Vec<(usize, usize)> = (200..3000).map(|u| (u, 7 * u)).collect();

    c.bench_function("Old SymbolTable: basic usage test", |b| {
        b.iter(|| {
            let mut old_table = OldSymbolTable::<usize, usize>::default();
            for item in layer1.iter() {
                old_table.insert(item.0, item.1);
            }
            for i in 0..20000 {
                black_box(old_table.get(&i));
            }
            old_table.push();
            for item in layer2.iter() {
                old_table.insert(item.0, item.1);
            }
            for i in 0..20000 {
                black_box(old_table.get(&i));
            }
            old_table.push();
            for i in 0..20000 {
                black_box(old_table.get(&i));
            }
            for item in layer3.iter() {
                old_table.insert(item.0, item.1);
            }
            for i in 0..20000 {
                black_box(old_table.get(&i));
            }
            old_table.pop();
            for i in 0..20000 {
                black_box(old_table.get(&i));
            }
            old_table.pop();
            for i in 0..20000 {
                black_box(old_table.get(&i));
            }
            for item in layer2_2.iter() {
                old_table.insert(item.0, item.1);
            }
            for i in 0..20000 {
                black_box(old_table.get(&i));
            }
            old_table.push();
            for i in 0..20000 {
                black_box(old_table.get(&i));
            }
            for item in layer3_2.iter() {
                old_table.insert(item.0, item.1);
            }
            for i in 0..20000 {
                black_box(old_table.get(&i));
            }
            old_table.pop();
            for i in 0..20000 {
                black_box(old_table.get(&i));
            }
            old_table.pop();
            for i in 0..20000 {
                black_box(old_table.get(&i));
            }
            std::mem::drop(old_table)
        })
    });
}

criterion_group!(benches, layer_benchmarks, insertion_benchmarks);
criterion_main!(benches);
