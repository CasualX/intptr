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

This crate's purpose is to model 32-bit and 64-bit 'pointers' to memory outside of your address space.

Eg. when interacting with other processes' memory address space.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
