/*!
Utility functions for testing `SymbolMap` implementations
*/
use super::*;

/// A basic test of symbol table functionality, starting from an empty symbol table
pub fn basic_symbol_table_test<S: SymbolMap<&'static str, Value = usize>>(symbols: &mut S) {
    assert!(symbols.is_empty());
    assert!(!symbols.contains_key("x"));
    assert!(!symbols.contains_key("y"));
    symbols.insert("x", 4);
    assert!(symbols.contains_key("x"));
    assert!(!symbols.contains_key("y"));
    assert_eq!(symbols.get("x"), Some(&4));
    symbols.insert("x", 3);
    assert_eq!(symbols.get("x"), Some(&3));
    assert!(symbols.contains_key("x"));
    assert!(!symbols.contains_key("y"));
    symbols.insert("y", 7);
    assert!(symbols.contains_key("x"));
    assert!(symbols.contains_key("y"));
    assert_eq!(symbols.get("y"), Some(&7));
    symbols.push();
    symbols.insert("x", 9);
    symbols.insert("z", 1);
    assert_eq!(symbols.get("x"), Some(&9));
    assert_eq!(symbols.get("y"), Some(&7));
    assert_eq!(symbols.get("z"), Some(&1));
    assert!(symbols.contains_key("z"));
    assert!(symbols.contains_key("x"));
    symbols.insert("z", 33);
    assert_eq!(symbols.get("x"), Some(&9));
    assert_eq!(symbols.get("y"), Some(&7));
    assert_eq!(symbols.get("z"), Some(&33));
    assert!(symbols.contains_key("z"));
    assert!(symbols.contains_key("x"));
    symbols.pop();
    assert!(!symbols.contains_key("z"))
}
