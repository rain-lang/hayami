/*!
Compare `SymbolTable` performance to other `HashMap`s
*/

use ahash::RandomState;
use benchmarks::old::SymbolTable as OldSymbolTable;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fxhash::FxHashMap;
use hayami::SymbolMap;
use im;
use im_rc;
use indexmap::IndexMap;
use rand::{thread_rng, Rng};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

pub fn insertion_benchmarks(c: &mut Criterion) {
    let mut rng = thread_rng();
    let mut table = hayami_im::SymbolTable::<usize, usize>::new();
    c.bench_function("hayami::SymbolTable: level 0 insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| table.insert(key, value))
    });
    std::mem::drop(table);
    let mut table = hayami_im::SymbolTable::<usize, usize>::new();
    c.bench_function("hayami_im::SymbolTable: level 0 insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| table.insert(key, value))
    });
    std::mem::drop(table);
    let mut table = hayami_im_rc::SymbolTable::<usize, usize>::new();
    c.bench_function("hayami_im_rc::SymbolTable: level 0 insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| table.insert(key, value))
    });
    std::mem::drop(table);
    let mut table = OldSymbolTable::<usize, usize>::new();
    c.bench_function("Old SymbolTable: level 0 insertion", |b| {
        let key = rng.gen::<usize>();
        let value = rng.gen::<usize>();
        b.iter(|| table.insert(key, value))
    });
    std::mem::drop(table);
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

pub fn sym_push<S, I, M>(table: &mut S, inputs: I, mut mappings: M)
where
    S: SymbolMap<I::Item>,
    I: IntoIterator,
    M: FnMut(&I::Item) -> S::Value,
{
    table.push();
    for item in inputs {
        let output = mappings(&item);
        table.insert(item, output);
    }
}

pub fn sym_read<S, I, T>(table: &mut S, inputs: I)
where
    S: SymbolMap<T>,
    I: IntoIterator,
    I::Item: Borrow<T>,
    T: Hash + Eq,
{
    table.pop();
    for item in inputs {
        black_box(table.get(item.borrow()));
    }
}
pub fn exercise_symbol_table<S: SymbolMap<usize, Value = usize> + Clone>(table: &mut S) {
    sym_push(table, 0..100, |x| 2 * x);
    sym_read(table, 10..150);
    sym_push(table, 0..200, |x| 3 * x);
    sym_read(table, 50..250);
    table.pop();
    sym_read(table, 50..300);
    sym_push(table, 50..300, |x| 4 * x);
    sym_read(table, 90..350);
    sym_push(table, 20..250, |x| 7 * x);
    sym_read(table, 80..300);
    table.pop();
    sym_push(table, 120..200, |x| 32 * x);
    table.pop();
    sym_push(table, 80..200, |x| 31 * x);
}

pub fn exercise_clone_symbol_table<S: SymbolMap<usize, Value = usize> + Clone>(table: &mut S) {
    sym_push(table, 0..100, |x| 2 * x);
    sym_push(table, 0..50, |x| 3 * x);
    sym_read(table, 10..150);
    for i in 0..5 {
        let mut table_2 = table.clone();
        sym_push(&mut table_2, 50..150, |x| (3 + i) * x);
        sym_read(&mut table_2, 20..110);
        for j in 0..5 {
            let mut table_3 = table_2.clone();
            sym_push(&mut table_3, 20..120, |x| j * (4 + i) * x);
            sym_read(&mut table_3, 10..100);
            table_3.pop();
            sym_read(&mut table_3, 20..110);
        }
        table_2.pop();
        sym_read(&mut table_2, 20..110);
        table_2.pop();
        sym_read(&mut table_2, 20..110);
    }
}

pub fn layer_benchmarks(c: &mut Criterion) {
    c.bench_function("hayami::SymbolTable: basic usage test", |b| {
        b.iter(|| {
            let mut table = hayami::SymbolTable::<usize, usize>::default();
            exercise_symbol_table(&mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("hayami_im::SymbolTable: basic usage test", |b| {
        b.iter(|| {
            let mut table = hayami_im::SymbolTable::<usize, usize>::default();
            exercise_symbol_table(&mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("hayami_im_rc::SymbolTable: basic usage test", |b| {
        b.iter(|| {
            let mut table = hayami_im_rc::SymbolTable::<usize, usize>::default();
            exercise_symbol_table(&mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Old SymbolTable: basic usage test", |b| {
        b.iter(|| {
            let mut table = OldSymbolTable::<usize, usize>::default();
            exercise_symbol_table(&mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("hayami::SymbolTable: clone usage test", |b| {
        b.iter(|| {
            let mut table = hayami::SymbolTable::<usize, usize>::default();
            exercise_clone_symbol_table(&mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("hayami_im::SymbolTable: clone usage test", |b| {
        b.iter(|| {
            let mut table = hayami_im::SymbolTable::<usize, usize>::default();
            exercise_clone_symbol_table(&mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("hayami_im_rc::SymbolTable: clone usage test", |b| {
        b.iter(|| {
            let mut table = hayami_im_rc::SymbolTable::<usize, usize>::default();
            exercise_clone_symbol_table(&mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Old SymbolTable: clone usage test", |b| {
        b.iter(|| {
            let mut table = OldSymbolTable::<usize, usize>::default();
            exercise_clone_symbol_table(&mut table);
            std::mem::drop(table)
        })
    });
}

criterion_group!(benches, layer_benchmarks, insertion_benchmarks);
criterion_main!(benches);
