# Tokio / Serde bindings for bincode

[![Build Status](https://travis-ci.org/luben/tokio-serde-bincode.svg?branch=master)](https://travis-ci.org/luben/tokio-serde-bincode)
[![crates.io](https://meritbadge.herokuapp.com/tokio-serde-bincode)](https://crates.io/crates/tokio-serde-bincode)
[![Docs](https://docs.rs/tokio-serde-bincode/badge.svg)](https://docs.rs/tokio-serde-bincode)

Utilities needed to easily implement a Tokio [Bincode] transport using [serde]
for serialization and deserialization of frame values. Based on [tokio-serde].

[bincode]: https://github.com/TyOverby/bincode
[serde]: https://serde.rs
[tokio-serde]: https://github.com/carllerche/tokio-serde

## Usage

To use `tokio-serde-bincode`, first add this to your `Cargo.toml`:

```toml
[dependencies]
tokio-serde-bincode = "0.2"
```

Next, add this to your crate:

```rust
extern crate tokio_serde_bincode;

use tokio_serde_bincode::{ReadBincode, WriteBincode};
```

# License

`tokio-serde-bincode` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
