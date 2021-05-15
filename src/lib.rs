/*! Pure Rust, high performance implementation of LZ4 compression.

A detailed explanation of the algorithm can be found [here](http://ticki.github.io/blog/how-lz4-works/).


# Examples
```
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
let input: &[u8] = b"Hello people, what's up?";
let compressed = compress_prepend_size(input);
let uncompressed = decompress_size_prepended(&compressed).unwrap();
assert_eq!(input, uncompressed);

```

## Feature Flags

- `safe-encode` uses only safe rust for encode. _enabled by default_
- `safe-decode` uses only safe rust for encode. _enabled by default_
- `checked-decode` will add additional checks if `safe-decode` is not enabled, to avoid out of bounds access. This should be enabled for untrusted input.
- `frame` support for LZ4 frame format. _implies `std`, enabled by default_
- `std` enables dependency on the standard library. _enabled by default_

For maximum performance use `no-default-features`.

*/

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg_attr(test, macro_use)]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate more_asserts;

pub mod block;
#[cfg(feature = "frame")]
pub mod frame;

pub use block::{compress, compress_into, compress_prepend_size};

pub use block::{decompress, decompress_into, decompress_size_prepended};
