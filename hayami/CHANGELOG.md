# 0.3.1

- Removed unnecessary dependencies and features

# 0.3.0

- Split into the `hayami` (this crate), `hayami-im`, `hayami-im-rc`, and `symbolmap-trait` crates.

# 0.2.1

- Added `with_capacity`, `with_hasher`, `with_capacity_and_hasher` constructors to the `fast` symbol table, and `with_hasher` constructors to the `snap` and `local` symbol tables.
- Added `Eq`, `PartialEq`, and `Hash` implementations

# 0.2.0

- Now provides a generic `SymbolMap<K>` trait with three feature-enabled implementors: `fast` (for speed), `snap` (for multithreaded O(1) cloning), and `local` (for single-threaded O(1) cloning).

# 0.1.1

- Added `get_defs` method