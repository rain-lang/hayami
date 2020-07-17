/*!
A symbol table implementation optimized for speed.
*/
use super::*;
use indexmap::IndexMap;
use std::fmt::{self, Debug, Formatter};

/// A symbol table implementation optimized for speed
#[derive(Clone)]
pub struct SymbolTable<K: Hash + Eq, V, S: BuildHasher = RandomState> {
    symbols: IndexMap<K, Vec<V>, S>,
    depth: usize,
    insertion_ix: usize,
    insertions: Vec<isize>,
}

impl<K: Hash + Eq, V, S: BuildHasher + Default> Default for SymbolTable<K, V, S> {
    #[inline]
    fn default() -> SymbolTable<K, V, S> {
        SymbolTable {
            symbols: IndexMap::default(),
            depth: 0,
            insertion_ix: 0,
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
        let len = self.symbols.len();
        let entry = self.symbols.entry(key);
        let ix = entry.index();
        entry.or_default().push(value);
        if self.depth != 0 {
            if ix == len {
                self.insertions[self.insertion_ix] -= 1;
            } else {
                self.insertions.push(ix as isize)
            }
        }
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
                while insertion < -1 {
                    insertion += 1;
                    self.symbols.pop();
                }
            }
            if let Some((_, entry)) = self.symbols.get_index_mut(insertion as usize) {
                entry.pop();
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
