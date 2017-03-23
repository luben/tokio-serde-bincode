# Tokio / Serde bindings for bincode

[![Build Status](https://travis-ci.org/alexcrichton/tokio-serde-bincode.svg?branch=master)](https://travis-ci.org/alexcrichton/tokio-serde-bincode)

Utilities needed to easily implement a Tokio [Bincode] transport using [serde]
for serialization and deserialization of frame values.

[Documentation](http://alexcrichton.com/tokio-serde-bincode)

[bincode]: https://github.com/TyOverby/bincode

## Usage

To use `tokio-serde-bincode`, first add this to your `Cargo.toml`:

```toml
[dependencies]
tokio-serde-bincode = { git = "https://github.com/alexcrichton/tokio-serde-bincode" }
```

Next, add this to your crate:

```rust
extern crate tokio_serde_bincode;

use tokio_serde_bincode::{ReadBincode, WriteBincode};
```

[serde]: https://serde.rs

# License

`tokio-serde-bincode` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), with portions covered by various
BSD-like licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
