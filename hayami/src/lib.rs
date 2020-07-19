/*!
Simple, general-use symbol table implementations with optional support for more advanced features
*/
use std::borrow::Borrow;
use std::hash::Hash;

#[cfg(feature = "fast")]
pub mod fast;

pub use symbolmap_trait::*;

#[cfg(test)]
mod testing;

/// The default random state in use
#[allow(unused)]
type RandomState = ahash::RandomState;