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
    prev: Option<Rc<SymbolTable<K, V, S>>>
}

impl<K: Hash + Eq, V, S: BuildHasher + Default> Default for SymbolTable<K, V, S> {
    fn default() -> SymbolTable<K, V, S> {
        SymbolTable {
            symbols: HashMap::default(),
            depth: 0,
            prev: None
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

impl<K: Hash + Eq + Clone, V: Clone, S: BuildHasher> SymbolTable<K, V, S> {
    /// Get a new symbol table extending this one
    fn extend(self) -> SymbolTable<K, V, S> {
        let symbols = self.symbols.clone();
        let depth = self.depth + 1;
        SymbolTable {
            symbols, depth, prev: Some(Rc::new(self))
        }
    }
}