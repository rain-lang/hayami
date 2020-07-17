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
use hayami::SymbolMap;

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

pub fn exercise_symbol_push_1<S: SymbolMap<usize, Value=usize>>(layers: &Layers, table:&mut S) {
    for item in layers.layer1.iter() {
        table.insert(item.0, item.1);
    }
    for i in 0..20000 {
        black_box(table.get(&i));
    }
    table.push();
    for item in layers.layer2.iter() {
        table.insert(item.0, item.1);
    }
    for i in 0..20000 {
        black_box(table.get(&i));
    }
    table.push();
    for i in 0..20000 {
        black_box(table.get(&i));
    }
    for item in layers.layer3.iter() {
        table.insert(item.0, item.1);
    }
    for i in 0..20000 {
        black_box(table.get(&i));
    }
}

pub fn exercise_symbol_pop_1<S: SymbolMap<usize, Value=usize>>(table:&mut S) {
    table.pop();
    for i in 0..20000 {
        black_box(table.get(&i));
    }
    table.pop();
    for i in 0..20000 {
        black_box(table.get(&i));
    }
}

pub fn exercise_symbol_push_2<S: SymbolMap<usize, Value=usize>>(layers: &Layers, table:&mut S) {
    for item in layers.layer2_2.iter() {
        table.insert(item.0, item.1);
    }
    for i in 0..20000 {
        black_box(table.get(&i));
    }
    table.push();
    for i in 0..20000 {
        black_box(table.get(&i));
    }
    for item in layers.layer3_2.iter() {
        table.insert(item.0, item.1);
    }
    for i in 0..20000 {
        black_box(table.get(&i));
    }
}

pub fn exercise_symbol_pop_2<S: SymbolMap<usize, Value=usize>>(table:&mut S) {
    table.pop();
    for i in 0..20000 {
        black_box(table.get(&i));
    }
    table.pop();
    for i in 0..20000 {
        black_box(table.get(&i));
    }
}

pub fn exercise_symbol_table<S: SymbolMap<usize, Value=usize>>(layers: &Layers, table: &mut S) {
    exercise_symbol_push_1(layers, table);
    exercise_symbol_pop_1(table);
    exercise_symbol_push_2(layers, table);
    exercise_symbol_pop_2(table);
}

pub fn exercise_clone_symbol_table<S: SymbolMap<usize, Value=usize> + Clone>(layers: &Layers, table: &mut S) {
    exercise_symbol_push_1(layers, table);
    let mut table_2 = table.clone();
    exercise_symbol_push_2(layers, &mut table_2);
    let mut table_3 = table_2.clone();
    exercise_symbol_push_1(layers, &mut table_2);
    exercise_symbol_pop_1(table);
    exercise_symbol_push_2(layers, &mut table_3);
    let mut table_4 = table_2.clone();
    exercise_symbol_pop_1(&mut table_2);
    exercise_symbol_pop_2(table);
    exercise_symbol_push_2(layers, &mut table_4);
    exercise_symbol_pop_2(&mut table_4);
    exercise_symbol_push_1(layers, &mut table_4);
}

pub fn exercise_loop_symbol_table<S: SymbolMap<usize, Value=usize> + Clone>(layers: &Layers, table: &mut S) {
    exercise_symbol_push_1(layers, table);
    for _ in 0..10 {
        let mut table2 = table.clone();
        exercise_symbol_push_2(layers, &mut table2);
        for _ in 0..10 {
            let mut table3 = table.clone();
            exercise_symbol_push_1(layers, &mut table3);
        }
    }
}

pub struct Layers {
    layer1: Vec<(usize, usize)>,
    layer2: Vec<(usize, usize)>,
    layer3: Vec<(usize, usize)>,
    layer2_2: Vec<(usize, usize)>,
    layer3_2: Vec<(usize, usize)>,
}

pub fn layer_benchmarks(c: &mut Criterion) {
    let layer1 = (0..10000).map(|u| (u, 2 * u)).collect();
    let layer2 = (250..750).map(|u| (u, 3 * u)).collect();
    let layer3 = (500..7000).map(|u| (u, 2 * u)).collect();
    let layer2_2 = (100..9000).map(|u| (u, u)).collect();
    let layer3_2 = (200..3000).map(|u| (u, 7 * u)).collect();
    let layers = Layers {
        layer1, layer2, layer3, layer2_2, layer3_2
    };
    c.bench_function("Fast SymbolTable: basic usage test", |b| {
        b.iter(|| {
            let mut table = hayami::fast::SymbolTable::<usize, usize>::default();
            exercise_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Snap SymbolTable: basic usage test", |b| {
        b.iter(|| {
            let mut table = hayami::snap::SymbolTable::<usize, usize>::default();
            exercise_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Local SymbolTable: basic usage test", |b| {
        b.iter(|| {
            let mut table = hayami::local::SymbolTable::<usize, usize>::default();
            exercise_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Old SymbolTable: basic usage test", |b| {
        b.iter(|| {
            let mut table = OldSymbolTable::<usize, usize>::default();
            exercise_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Fast SymbolTable: clone usage test", |b| {
        b.iter(|| {
            let mut table = hayami::fast::SymbolTable::<usize, usize>::default();
            exercise_clone_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Snap SymbolTable: clone usage test", |b| {
        b.iter(|| {
            let mut table = hayami::snap::SymbolTable::<usize, usize>::default();
            exercise_clone_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Local SymbolTable: clone usage test", |b| {
        b.iter(|| {
            let mut table = hayami::local::SymbolTable::<usize, usize>::default();
            exercise_clone_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Old SymbolTable: clone usage test", |b| {
        b.iter(|| {
            let mut table = OldSymbolTable::<usize, usize>::default();
            exercise_clone_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Fast SymbolTable: clone loop usage test", |b| {
        b.iter(|| {
            let mut table = hayami::fast::SymbolTable::<usize, usize>::default();
            exercise_loop_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Snap SymbolTable: clone loop usage test", |b| {
        b.iter(|| {
            let mut table = hayami::snap::SymbolTable::<usize, usize>::default();
            exercise_loop_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Local SymbolTable: clone loop usage test", |b| {
        b.iter(|| {
            let mut table = hayami::local::SymbolTable::<usize, usize>::default();
            exercise_loop_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
    c.bench_function("Old SymbolTable: clone loop usage test", |b| {
        b.iter(|| {
            let mut table = OldSymbolTable::<usize, usize>::default();
            exercise_loop_symbol_table(&layers, &mut table);
            std::mem::drop(table)
        })
    });
}

criterion_group!(benches, layer_benchmarks, insertion_benchmarks);
criterion_main!(benches);
