/*!
A simple, general-use symbol table optimized for speed at the cost of some advanced features.
*/
#![deny(missing_docs, unsafe_code, missing_debug_implementations)]

use ahash::RandomState;
use indexmap::IndexMap;
use std::borrow::Borrow;
use std::fmt::{self, Debug, Formatter};
use std::hash::BuildHasher;
use std::hash::Hash;

pub use symbolmap_trait::SymbolMap;

/// A symbol table implementation optimized for speed
#[derive(Clone)]
pub struct SymbolTable<K: Hash + Eq, V, S: BuildHasher = RandomState> {
    symbols: IndexMap<K, Vec<V>, S>,
    depth: usize,
    insertion_ix: usize,
    defined: usize,
    insertions: Vec<isize>,
}

impl<K: Hash + Eq, V, S: BuildHasher + Default> Default for SymbolTable<K, V, S> {
    #[inline]
    fn default() -> SymbolTable<K, V, S> {
        SymbolTable {
            symbols: IndexMap::default(),
            depth: 0,
            insertion_ix: 0,
            defined: 0,
            insertions: Vec::new(),
        }
    }
}

impl<K: Hash + Eq, V, S: BuildHasher + Default> SymbolTable<K, V, S> {
    /// Create a new symbol table with the given `BuildHasher`
    #[inline]
    pub fn with_hasher(hash_builder: S) -> SymbolTable<K, V, S> {
        SymbolTable {
            symbols: IndexMap::with_hasher(hash_builder),
            depth: 0,
            insertion_ix: 0,
            defined: 0,
            insertions: Vec::new(),
        }
    }
    /// Create a new symbol table having the given capacity with the given `BuildHasher`
    #[inline]
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> SymbolTable<K, V, S> {
        SymbolTable {
            symbols: IndexMap::with_capacity_and_hasher(capacity, hash_builder),
            depth: 0,
            insertion_ix: 0,
            defined: 0,
            insertions: Vec::new(),
        }
    }
}

impl<K: Hash + Eq, V> SymbolTable<K, V> {
    /// Create a new symbol table having the given capacity
    #[inline]
    pub fn with_capacity(n: usize) -> SymbolTable<K, V> {
        SymbolTable {
            symbols: IndexMap::with_capacity_and_hasher(n, RandomState::default()),
            depth: 0,
            insertion_ix: 0,
            defined: 0,
            insertions: Vec::new(),
        }
    }
    /// Create a new, empty symbol table
    #[inline]
    pub fn new() -> SymbolTable<K, V> {
        Self::default()
    }
}

impl<K: Hash + Eq, V: PartialEq, S: BuildHasher> PartialEq for SymbolTable<K, V, S> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth
            && self.insertion_ix == other.insertion_ix
            && self.defined == other.defined
            && self.insertions == other.insertions
            && self.symbols == other.symbols
    }
}

impl<K: Hash + Eq, V: Eq, S: BuildHasher> Eq for SymbolTable<K, V, S> {}

impl<K: Hash + Eq + Debug, V: Debug, S: BuildHasher> Debug for SymbolTable<K, V, S> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.debug_struct("SymbolTable")
            .field("symbols", &self.symbols)
            .field("depth", &self.depth)
            .field("insertions", &self.insertions)
            .finish()
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> SymbolMap<K> for SymbolTable<K, V, S> {
    type Value = V;
    #[inline]
    fn insert(&mut self, key: K, value: Self::Value) {
        let len = self.symbols.len();
        let entry = self.symbols.entry(key);
        let ix = entry.index();
        let entry = entry.or_default();
        let entry_len = entry.len();
        entry.push(value);
        if self.depth != 0 {
            if ix == len && entry_len == 0 {
                self.insertions[self.insertion_ix] -= 1;
            } else {
                self.insertions.push(ix as isize)
            }
        }
        self.defined += 1;
    }
    #[inline]
    fn get<Q>(&self, key: &Q) -> Option<&Self::Value>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        let vec = self.symbols.get(key)?;
        vec.last()
    }
    #[inline]
    fn try_get_mut<Q>(&mut self, _key: &Q) -> Option<&mut Self::Value>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        None
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.defined == 0
    }
    #[inline]
    fn push(&mut self) {
        self.insertion_ix = self.insertions.len();
        self.insertions.push(-1);
        self.depth += 1;
    }
    #[inline]
    fn pop(&mut self) {
        if self.depth == 0 {
            return;
        }
        self.depth -= 1;
        while let Some(mut insertion) = self.insertions.pop() {
            if insertion < 0 {
                let undefined = (-insertion as usize) - 1;
                self.defined -= undefined;
                while insertion < -1 {
                    insertion += 1;
                    self.symbols.pop();
                }
                return;
            }
            if let Some((_, entry)) = self.symbols.get_index_mut(insertion as usize) {
                entry.pop();
                self.defined -= 1;
            }
        }
        for (i, insertion) in self.insertions.iter().enumerate().rev() {
            if *insertion < 0 {
                self.insertion_ix = i;
                break;
            }
        }
    }
    #[inline]
    fn depth(&self) -> usize {
        self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use symbolmap_trait::testing;
    #[test]
    fn basic_symbol_table_test() {
        testing::basic_symbol_table_test(&mut SymbolTable::new())
    }
    #[test]
    fn inserting_back_twice_works() {
        let mut table = SymbolTable::<usize, usize>::new();
        table.insert(5, 3);
        table.push();
        table.insert(5, 4);
        table.push();
        table.insert(5, 5);
        assert_eq!(table.get(&5), Some(&5));
        table.pop();
        assert_eq!(table.get(&5), Some(&4));
        table.pop();
        assert_eq!(table.get(&5), Some(&3));
        table.pop();
    }
}
