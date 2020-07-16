/*!
Simple, general-use symbol table implementations with optional support for more advanced features
*/
use std::borrow::Borrow;

pub mod local;
pub mod snap;

/**
A trait for a simple, general-use symbol table which can be indexed by a given key.

Behaves like a stack of `HashMap`s.
*/
pub trait SymbolTable<K> {
    /// The value stored in this symbol table
    type Value;
    /// Insert a key/value pair into this symbol table at the current level
    pub fn insert(&mut self, key: K, value: Self::Value);
    /// Get the most recent definition of a key in this symbol table
    pub fn get(&self, key: &Q) -> Option<&Self::Value> where Q: Borrow<K>;
    /// Try to get a mutable reference to the definition of a key in the top level of this symbol table
    /// 
    /// May fail for arbitrary reasons, to avoid, e.g., re-inserting the key at the top level as mutable.
    pub fn try_get_mut(&mut self, key: &Q) -> Option<&mut Self::Value> where Q: Borrow<K>;
    /// Push a level onto this symbol table
    pub fn push(&mut self);
    /// Pop a level from this symbol table
    /// 
    /// Note that this is *not* guaranteed to drop all elements stored in the level!
    pub fn pop(&mut self);
}

/**
A trait for a simple, general-use symbol table which can be indexed by a given key in which entries may be infallibly mutated.

Behaves like a stack of `HashMap`s.
*/
pub trait MutSymbolTable<K>: SymbolTable<K> {
    /// Get a mutable reference to the definition of a key in the top level of this symbol table
    pub fn get_mut(&mut self, key: &Q) -> Option<&mut Self::Value> where Q: Borrow<K> {
        self.try_get_mut(key)
    }
}