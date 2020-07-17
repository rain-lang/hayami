# 0.2.0

- Now provides a generic `SymbolMap<K>` trait with three feature-enabled implementors: `fast` (for speed), `im` (for multithreaded O(1) cloning), and `im-rc` (for single-threaded O(1) cloning).

# 0.1.1

- Added `get_defs` method