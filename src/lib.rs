/*!
A simple, general use symbol table, designed specifically for use in the implementation of the [`rain` language](https://gitlab.com/tekne/rain).
*/
use ahash::RandomState;
use indexmap::{Equivalent, IndexMap};
use std::default::Default;
use std::fmt::{self, Debug, Formatter};
use std::hash::{BuildHasher, Hash};

/**
A simple, generic symbol table.

Behaves like a stack of `HashMap`s, where you can only insert symbols into the top map with `insert`. When a symbol is
looked up with `get`, first the top map is checked, then the map under it, etc., with the first match found returned.
This is implemented more efficiently using an `IndexMap`. `get_full` does the same, but returns the depth at which the
symbol was found.
*/
#[derive(Clone, Eq, PartialEq)]
pub struct SymbolTable<K: Hash + Eq, V, S: BuildHasher = RandomState> {
    symbols: IndexMap<K, Vec<(V, usize)>, S>,
    scopes: Vec<Vec<usize>>,
}

impl<K: Hash + Eq + Debug, V: Debug, S: BuildHasher> Debug for SymbolTable<K, V, S> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        fmt.debug_struct("SymbolTable")
            .field("symbols", &self.symbols)
            .field("scopes", &self.scopes)
            .finish()
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> SymbolTable<K, V, S>
where
    IndexMap<K, Vec<(V, usize)>, S>: Default,
{
    /// Create a new, empty symbol table
    pub fn new() -> SymbolTable<K, V, S> {
        Self::default()
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> SymbolTable<K, V, S>
where
    S: Default,
{
    /// Create a symbol table with a given capacity
    pub fn with_capacity(n: usize) -> SymbolTable<K, V, S> {
        Self::with_capacity_and_hasher(n, S::default())
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> Default for SymbolTable<K, V, S>
where
    IndexMap<K, Vec<(V, usize)>, S>: Default,
{
    fn default() -> SymbolTable<K, V, S> {
        SymbolTable {
            symbols: IndexMap::default(),
            scopes: vec![Vec::new()],
        }
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> SymbolTable<K, V, S> {
    /// Create a symbol table with a given hasher
    pub fn with_hasher(hash_builder: S) -> SymbolTable<K, V, S> {
        SymbolTable {
            symbols: IndexMap::with_hasher(hash_builder),
            scopes: vec![Vec::new()],
        }
    }
    /// Create a symbol table with a given capacity and hasher
    pub fn with_capacity_and_hasher(n: usize, hash_builder: S) -> SymbolTable<K, V, S> {
        SymbolTable {
            symbols: IndexMap::with_capacity_and_hasher(n, hash_builder),
            scopes: vec![Vec::new()],
        }
    }
    /// Get the current depth
    pub fn depth(&self) -> usize {
        self.scopes.len() - 1
    }
    /// Register a given symbol at the current depth, returning the current definition at
    /// the current depth, if any.
    pub fn insert(&mut self, key: K, mut value: V) -> Option<V> {
        let depth = self.depth();
        let entry = self.symbols.entry(key);
        let index = entry.index();
        let v = entry.or_insert_with(Vec::new);
        if let Some((old_value, old_depth)) = v.last_mut() {
            if depth == *old_depth {
                std::mem::swap(old_value, &mut value);
                return Some(value);
            }
        }
        v.push((value, depth));
        self.scopes.last_mut().unwrap().push(index);
        None
    }
    /// Try to register a given symbol at the current depth. Fail if the symbol is already defined
    pub fn try_insert(&mut self, key: K, value: V) -> Result<(), V> {
        let depth = self.depth();
        let entry = self.symbols.entry(key);
        let index = entry.index();
        let v = entry.or_insert_with(Vec::new);
        if let Some((_, old_depth)) = v.last_mut() {
            if depth == *old_depth {
                return Err(value);
            }
        }
        v.push((value, depth));
        self.scopes.last_mut().unwrap().push(index);
        Ok(())
    }
    /// Get the definition of a current symbol, along with its depth, if any
    pub fn get_defs<Q>(&self, key: &Q) -> &[(V, usize)]
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.symbols.get(key).map(|vec| &vec[..]).unwrap_or(&[])
    }
    /// Get the definition of a current symbol, along with its depth, if any
    pub fn get_full<Q>(&self, key: &Q) -> Option<(&V, usize)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.get_defs(key).last().map(|(v, d)| (v, *d))
    }
    /// Get the definition of a symbol
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.get_full(key).map(|(v, _)| v)
    }
    /// Check whether a symbol has a definition
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.symbols
            .get(key)
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }
    /// Mutably get the definition of a current symbol, along with its depth, if any
    pub fn get_full_mut<Q>(&mut self, key: &Q) -> Option<(&mut V, usize)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.symbols
            .get_mut(key)
            .map(|v| v.last_mut().map(|(v, d)| (v, *d)))
            .flatten()
    }
    /// Try to mutably get the definition of a current symbol at the current depth
    pub fn try_get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        let curr_depth = self.depth();
        if let Some((value, depth)) = self.get_full_mut(key) {
            if depth == curr_depth {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    }
    //TODO: get_mut
    /// Get the mutable definition of a current symbol, along with its depth, if any
    /// Jump to a given depth, removing obsolete definitions.
    /// Return the number of keys and definitions removed, as well as keys touched, if any.
    pub fn jump(&mut self, depth: usize) {
        let target = depth + 1;
        while target > self.scopes.len() {
            self.scopes.push(Vec::new());
        }
        while self.scopes.len() > target {
            for ix in self.scopes.pop().unwrap() {
                let (_, v) = if let Some(v) = self.symbols.get_index_mut(ix) {
                    v
                } else {
                    continue;
                };
                v.pop();
            }
        }
    }
    /// Add a level of depth
    #[inline]
    pub fn push(&mut self) {
        self.jump(self.depth() + 1);
    }
    /// Pop up to `n` levels of depth.
    #[inline]
    pub fn popn(&mut self, n: usize) {
        self.jump(self.depth().saturating_sub(n))
    }
    /// Try to remove a level of depth. Does nothing if depth  = 0
    #[inline]
    pub fn pop(&mut self) {
        self.jump(self.depth().saturating_sub(1))
    }
    /// Check whether a symbol table is empty
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
    /// Reserve space for at least `additional` symbols in this symbol table
    pub fn reserve(&mut self, additional: usize) {
        self.symbols.reserve(additional)
    }
    /// Get the capacity of this symbol table
    pub fn capacity(&self) -> usize {
        self.symbols.capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ahash::AHasher;
    use pretty_assertions::assert_eq;
    use std::hash::BuildHasherDefault;

    fn symbol_table_test<S: BuildHasher>(mut symbols: SymbolTable<&str, usize, S>) {
        assert!(symbols.is_empty());
        assert!(!symbols.contains_key("x"));
        assert!(!symbols.contains_key("y"));
        symbols.insert("x", 3);
        assert!(symbols.contains_key("x"));
        assert!(!symbols.contains_key("y"));
        symbols.insert("y", 7);
        assert!(symbols.contains_key("x"));
        assert!(symbols.contains_key("y"));
        symbols.push();
        assert_eq!(symbols.insert("x", 9), None);
        assert_eq!(symbols.insert("z", 1), None);
        assert_eq!(symbols.get_full("x"), Some((&9, 1)));
        assert_eq!(symbols.get_full("y"), Some((&7, 0)));
        assert_eq!(symbols.get_full("z"), Some((&1, 1)));
        assert_eq!(symbols.get("x"), Some(&9));
        assert_eq!(symbols.get("y"), Some(&7));
        assert_eq!(symbols.get("z"), Some(&1));
        assert_eq!(symbols.try_get_mut("x"), Some(&mut 9));
        assert_eq!(symbols.try_get_mut("y"), None);
        assert_eq!(symbols.try_get_mut("z"), Some(&mut 1));
        assert!(symbols.contains_key("z"));
        assert!(symbols.contains_key("x"));
        assert_eq!(symbols.insert("z", 33), Some(1));
        assert_eq!(symbols.get_full("x"), Some((&9, 1)));
        assert_eq!(symbols.get_full("y"), Some((&7, 0)));
        assert_eq!(symbols.get_full("z"), Some((&33, 1)));
        assert_eq!(symbols.get("x"), Some(&9));
        assert_eq!(symbols.get("y"), Some(&7));
        assert_eq!(symbols.get("z"), Some(&33));
        assert_eq!(symbols.try_get_mut("x"), Some(&mut 9));
        assert_eq!(symbols.try_get_mut("y"), None);
        assert_eq!(symbols.try_get_mut("z"), Some(&mut 33));
        assert!(symbols.contains_key("z"));
        assert!(symbols.contains_key("x"));
        symbols.pop();
        assert_eq!(symbols.get_full("x"), Some((&3, 0)));
        assert_eq!(symbols.get_full("y"), Some((&7, 0)));
        assert_eq!(symbols.get_full("z"), None);
        assert_eq!(symbols.try_get_mut("x"), Some(&mut 3));
        assert_eq!(symbols.try_get_mut("y"), Some(&mut 7));
        assert_eq!(symbols.try_get_mut("z"), None);
        assert!(!symbols.contains_key("z"))
    }

    const TEST_CAPACITY: usize = 52;
    const TEST_RESERVE: usize = 100;

    #[test]
    fn two_layer_symbol_table_works() {
        let symbols = SymbolTable::<_, _>::new();
        symbol_table_test(symbols);
        let mut symbols = SymbolTable::<_, _>::with_capacity(TEST_CAPACITY);
        assert!(symbols.capacity() >= TEST_CAPACITY);
        symbols.reserve(TEST_RESERVE);
        // Not fully implemented for `IndexMap`
        /*
        assert!(
            symbols.capacity() >= TEST_RESERVE,
            "Reserved {}, but got capacity {}",
            TEST_RESERVE,
            symbols.capacity()
        );
        */
        symbol_table_test(symbols);
    }

    #[test]
    fn default_ahasher_symbol_table_works() {
        let symbols = SymbolTable::with_hasher(BuildHasherDefault::<AHasher>::default());
        symbol_table_test(symbols);
        let mut symbols = SymbolTable::with_capacity_and_hasher(
            TEST_CAPACITY,
            BuildHasherDefault::<AHasher>::default(),
        );
        assert!(symbols.capacity() >= TEST_CAPACITY);
        symbols.reserve(TEST_RESERVE);
        /*
        // Not fully implemented for `IndexMap`
        assert!(
            symbols.capacity() >= TEST_RESERVE,
            "Reserved {}, but got capacity {}",
            TEST_RESERVE,
            symbols.capacity()
        );
        */
        symbol_table_test(symbols);
    }
}
