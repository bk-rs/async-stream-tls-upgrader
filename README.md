# async-stream-tls-upgrader

* [Cargo package](https://crates.io/crates/async-stream-tls-upgrader)

## Examples

### async-net 

* [async_tls_client](demos/async-net/src/async_tls_client.rs)
* [async_native_tls_client](demos/async-net/src/async_native_tls_client.rs)
* [imap_client](demos/async-net/src/imap_client.rs)
* [smtp_client](demos/async-net/src/smtp_client.rs)

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
