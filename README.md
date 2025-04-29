# QUIC Example

A simple example of a QUIC server and client using Rust using quinn.

## How to use

- Run the server using `cargo run --bin quic_server`. By default it listens on `127.0.0.1:4433` .
- Run the client using `cargo run --bin quic_client`.

```quic-monorepo/
├── quic_client
|
├── quic_server <- QUIC Server
|
├── tls_cert_test <- Rust-based TLS server
|
├── .gitignore
|
├── Cargo.toml
|
├── README.md
```
