# 0.2

- Changed `SymbolTable::new()` less ambiguous: it is now only defined for `S = ahash::RandomState`. Use `SymbolTable::default()` for other hashers.

# 0.1.1

- Added `get_defs` method