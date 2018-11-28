# v.0.2.0

## Breaking chages

- Upgraded dependency to tokio-serde-0.3. It swaps Bytes/BytesMut in
the signatures of the Read/Write parts.

- Removed the Error type and we are reusing instead the bincode::Error that
already has a slot for the Io errors.

- Upgrade to bincode-1.0

## Organisational
- project transfered from @alexcrichton to @luben
- @luben granted co-ownership on the crates.io by @antoyo