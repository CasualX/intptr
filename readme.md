IntPtr
======

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/intptr.svg)](https://crates.io/crates/intptr)
[![docs.rs](https://docs.rs/intptr/badge.svg)](https://docs.rs/intptr)
[![Build status](https://github.com/CasualX/intptr/workflows/Check/badge.svg)](https://github.com/CasualX/intptr/actions)

Explicitly sized, typed pointers to memory outside the current address space.

Library
-------

This library is available on [crates.io](https://crates.io/crates/intptr).

Documentation can be found on [docs.rs](https://docs.rs/intptr/).

In your Cargo.toml, put

```text
[dependencies]
intptr = "1.0"
```

Examples
--------

`IntPtr32` and `IntPtr64` model addresses independently of the host's pointer width. `IntPtr` is
an alias for the type matching the host.

```rust
use intptr::{IntPtr32, IntPtr64};

// Element arithmetic is scaled by the pointee type.
let array = IntPtr64::<[u32]>::from_raw(0x0000_7ff0_0000_1000);
let second = array.at(1);
assert_eq!(second.into_raw(), 0x0000_7ff0_0000_1004);

// `field` uses a byte offset and can change the pointee type.
let object = IntPtr32::<()>::from_raw(0x1000);
let field = object.field::<u16>(6);
assert_eq!(field, IntPtr32::<u16>::from_raw(0x1006));

// String parsing is hexadecimal, with or without a `0x` prefix.
assert_eq!("0x2000".parse(), Ok(IntPtr64::<()>::from_raw(0x2000)));
assert_eq!("2000".parse(), Ok(IntPtr64::<()>::from_raw(0x2000)));
```

The crate is `no_std` and supports Rust 1.85 and newer. Enable the optional `serde` feature for
integer serialization and deserialization, or `dataview` for `dataview::Pod` implementations.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
