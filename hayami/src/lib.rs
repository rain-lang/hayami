/*!
Simple, general-use symbol table implementations with optional support for more advanced features
*/
use std::borrow::Borrow;
use std::hash::Hash;

#[cfg(feature = "fast")]
pub mod fast;
#[cfg(feature = "local")]
pub mod local;

pub use symbolmap_trait::*;

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