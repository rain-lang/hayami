/*!
A symbol table implementation optimized for speed.
*/
use super::*;
use indexmap::IndexMap;
use std::fmt::{self, Formatter, Debug};

/// A symbol table implementation optimized for speed
#[derive(Clone)]
pub struct SymbolTable<K: Hash + Eq, V, S: BuildHasher = RandomState> {
    symbols: IndexMap<K, Vec<V>, S>,
    depth: usize,
    insertions: Vec<usize>,
}

impl<K: Hash + Eq, V, S: BuildHasher + Default> Default for SymbolTable<K, V, S> {
    #[inline]
    fn default() -> SymbolTable<K, V, S> {
        SymbolTable {
            symbols: IndexMap::default(),
            depth: 0,
            insertions: Vec::new(),
        }
    }
}

impl<K: Hash + Eq, V> SymbolTable<K, V> {
    #[inline]
    pub fn new() -> SymbolTable<K, V> {
        Self::default()
    }
}

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
        self.symbols.entry(key).or_default().push(value)
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
    fn try_get_mut<Q>(&mut self, key: &Q) -> Option<&mut Self::Value>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        let vec = self.symbols.get_mut(key)?;
        vec.last_mut()
    }
    #[inline]
    fn push(&mut self) {
        self.insertions.push(std::usize::MAX);
        self.depth += 1;
    }
    #[inline]
    fn pop(&mut self) {
        while let Some(insertion) = self.insertions.pop() {
            if insertion == std::usize::MAX { break; }
            if let Some((_, entry)) = self.symbols.get_index_mut(insertion) {
                entry.pop();
            }
        }
    }
    #[inline]
    fn depth(&self) -> usize {
        self.depth
    }
}