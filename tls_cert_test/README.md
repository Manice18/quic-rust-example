# TLS cert client-server

This is a Rust-based TLS server that demonstrates how to set up a secure server using the rustls library. It generates a self-signed TLS certificate at runtime, configures a secure server, and listens for incoming client connections over a specified port (127.0.0.1:8443 by default). The server handles multiple clients concurrently using threads and provides basic functionality to read and process data sent by clients over a secure TLS connection.

## How to Run

- In your terminal type `cargo run` (acts as server).
- Open another terminal(acts as client) and type `openssl s_client -connect 127.0.0.1:8443 -servername localhost`.
- Then type any message on the client and see the response on server.
