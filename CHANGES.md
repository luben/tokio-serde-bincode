# Changelog

## Next

## 0.2.1

### Features

- Added support for i128/u128. Enabled by the "i128" compile time feature.

### Documentation

- Added back reworked examples from v0.1.1

## 0.2.0

### Breaking chages

- Upgraded dependency to tokio-serde-0.3. It swaps Bytes/BytesMut in
the signatures of the Read/Write parts.

- Removed the Error type and we are reusing instead the bincode::Error that
already has a slot for the Io errors.

- Upgrade to bincode-1.0.

### Documentation

- Now the API documentation is available on docs.rs

### Organisational

- project transfered from @alexcrichton to @luben

- @luben granted co-ownership on the crates.io by @antoyo
