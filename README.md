# async-stream-tls-upgrader

* [Cargo package](https://crates.io/crates/async-stream-tls-upgrader)

## Examples

### smol 

* [async_tls_client](demos/smol/src/async_tls_client.rs)

## Dev

```
cargo test --all-features --all -- --nocapture && \
cargo clippy --all -- -D clippy::all && \
cargo fmt --all -- --check
```

```
cargo build-all-features
cargo test-all-features --all
```
