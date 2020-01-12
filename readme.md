IntPtr
======

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/intptr.svg)](https://crates.io/crates/intptr)
[![docs.rs](https://docs.rs/intptr/badge.svg)](https://docs.rs/intptr)

Unmanaged, explicitly sized and typed Pointers.

Library
-------

This library is available on [crates.io](https://crates.io/crates/intptr).

Documentation can be found on [docs.rs](https://docs.rs/intptr/).

In your Cargo.toml, put

```
[dependencies]
intptr = "0.1"
```

Examples
--------

This crate's purpose is to model 32-bit and 64-bit 'pointers' to memory outside of your address space.

Eg. when interacting with other processes' their memory address space.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
