/*!
Simple, general-use symbol table implementations with optional support for more advanced features
*/
use std::borrow::Borrow;
use std::hash::Hash;

#[cfg(feature = "fast")]
pub mod fast;
#[cfg(feature = "local")]
pub mod local;
#[cfg(feature = "snap")]
pub mod snap;

#[cfg(test)]
mod testing;

/// The default random state in use
#[allow(unused)]
type RandomState = ahash::RandomState;

/// The `Arc` in use
///
/// Supports `elysees` (default) or `std`
#[cfg(feature = "elysees")]
#[allow(unused)]
type Arc<T> = elysees::Arc<T>;
#[cfg(not(feature = "elysees"))]
#[allow(unused)]
type Arc<T> = std::sync::Arc<T>;

/// The `Rc` in use
///
/// Supports `std` only as of now
#[allow(unused)]
type Rc<T> = std::rc::Rc<T>;

/**
A trait for a symbol table which can be indexed by a given key.

Behaves like a stack of `HashMap`s.
*/
pub trait SymbolMap<K> {
    /// The value stored in this symbol table
    type Value;
    /// Insert a key/value pair into this symbol table at the current level
    fn insert(&mut self, key: K, value: Self::Value);
    /// Get the most recent definition of a key in this symbol table
    fn get<Q>(&self, key: &Q) -> Option<&Self::Value>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>;
    /// Whether this symbol table contains this key
    #[inline]
    fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        self.get(key).is_some()
    }
    /// Whether this symbol table is empty
    fn is_empty(&self) -> bool;
    /// Try to get a mutable reference to the definition of a key in the top level of this symbol table
    ///
    /// May fail for arbitrary reasons, to avoid, e.g., re-inserting the key at the top level as mutable.
    fn try_get_mut<Q>(&mut self, key: &Q) -> Option<&mut Self::Value>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>;
    /// Push a level onto this symbol table
    fn push(&mut self);
    /// Pop a level from this symbol table
    ///
    /// Note that this is *not* guaranteed to drop all elements stored in the level!
    fn pop(&mut self);
    /// Get the current depth of this symbol table
    fn depth(&self) -> usize;
}

/**
A trait for a symbol table which in which entries may be infallibly mutated.
*/
pub trait MutSymbolMap<K>: SymbolMap<K> {
    /// Get a mutable reference to the definition of a key in the top level of this symbol table
    #[inline(always)]
    fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Self::Value>
    where
        Q: ?Sized + Hash + Eq,
        K: Borrow<Q>,
    {
        self.try_get_mut(key)
    }
}

/**
A trait for a stack-like symbol table in which a reference to the previous layer may be obtained
*/
pub trait SymbolStack<K>: SymbolMap<K> {
    /// Get the previous layer of this symbol table
    fn prev(&self) -> Option<&Self>;
}
