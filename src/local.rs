/*!
A symbol table implementation supporting snapshots, i.e. an `O(1)` cloning operation.

Faster than the implementation in `snap`, at the cost of not implementing `Send` + `Sync`.
*/
use super::*;
use im_rc::HashMap;

/**
A symbol table implementation supporting snapshots, i.e. an `O(1)` cloning operation.

Faster than the implementation in `snap`, at the cost of not implementing `Send` + `Sync`.
*/
pub struct SymbolTable<K: Hash + Eq, V, S: BuildHasher = RandomState> {
    /// This layer of the symbol table
    symbols: HashMap<K, V, S>,
    /// The depth of this symbol table
    depth: usize,
    /// A link to the previous layer's table, forming a singly-linked list
    prev: Option<Rc<SymbolTable<K, V, S>>>,
}

impl<K: Hash + Eq, V, S: BuildHasher + Default> Default for SymbolTable<K, V, S> {
    #[inline]
    fn default() -> SymbolTable<K, V, S> {
        SymbolTable {
            symbols: HashMap::default(),
            depth: 0,
            prev: None,
        }
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> SymbolTable<K, V, S> {
    /// Get an `Arc` to the previous layer's table, if there is any
    #[inline]
    pub fn get_prev(&self) -> Option<&Rc<SymbolTable<K, V, S>>> {
        self.prev.as_ref()
    }
}

impl<K: Hash + Eq + Clone, V: Clone, S: BuildHasher> Clone for SymbolTable<K, V, S> {
    #[inline]
    fn clone(&self) -> SymbolTable<K, V, S> {
        SymbolTable {
            symbols: self.symbols.clone(),
            depth: self.depth,
            prev: self.prev.clone(),
        }
    }
}

impl<K: Hash + Eq + Clone, V: Clone, S: BuildHasher> SymbolTable<K, V, S> {
    /// Get a new symbol table extending this one
    #[inline]
    pub fn extend(self) -> SymbolTable<K, V, S> {
        let symbols = self.symbols.clone();
        let depth = self.depth + 1;
        SymbolTable {
            symbols,
            depth,
            prev: Some(Rc::new(self)),
        }
    }
}

impl<K: Hash + Eq + Clone, V: Clone, S: BuildHasher> SymbolMap<K> for SymbolTable<K, V, S> {
    type Value = V;
    #[inline]
    fn insert(&mut self, key: K, value: Self::Value) {
        self.symbols.insert(key, value);
    }
    #[inline]
    fn get<Q>(&self, key: &Q) -> Option<&Self::Value>
    where
        Q: Hash + Eq,
        K: Borrow<Q>,
    {
        self.symbols.get(key)
    }
    #[inline]
    fn try_get_mut<Q>(&mut self, key: &Q) -> Option<&mut Self::Value>
    where
        Q: Hash + Eq,
        K: Borrow<Q>,
    {
        self.symbols.get_mut(key)
    }
    #[inline]
    fn push(&mut self) {
        self.prev = Some(Rc::new(self.clone()));
        self.depth += 1;
    }
    #[inline]
    fn pop(&mut self) {
        if let Some(prev) = self.prev.as_deref() {
            *self = prev.clone();
        }
    }
    #[inline]
    fn depth(&self) -> usize {
        self.depth
    }
}

impl<K: Hash + Eq + Clone, V: Clone, S: BuildHasher> MutSymbolMap<K> for SymbolTable<K, V, S> {}

impl<K: Hash + Eq + Clone, V: Clone, S: BuildHasher> SymbolStack<K> for SymbolTable<K, V, S> {
    #[inline]
    fn prev(&self) -> Option<&Self> {
        self.prev.as_deref()
    }
}
