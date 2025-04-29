use quinn::{ClientConfig, Endpoint};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Create a QUIC endpoint
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse().unwrap()).unwrap();

    // Configure client to skip certificate verification (for self-signed certs)
    let client_crypto = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification {}))
        .with_no_client_auth();
    endpoint.set_default_client_config(ClientConfig::new(Arc::new(client_crypto)));

    // Connect to the server
    let connection = endpoint
        .connect("127.0.0.1:4433".parse().unwrap(), "localhost")
        .unwrap()
        .await
        .unwrap();

    info!("Connected to server!");

    // Open a bi-directional stream
    let (mut send, mut recv) = connection.open_bi().await.unwrap();

    let message = b"Hello from client!";
    send.write_all(message).await.unwrap();
    send.finish().await.unwrap();

    let mut buf = vec![0; 1024];
    let n = recv.read(&mut buf).await.unwrap().unwrap();
    let echoed = &buf[..n];

    info!("Received echoed: {}", String::from_utf8_lossy(echoed));
}

// Helper struct to skip server certificate verification (for local testing only)
struct SkipServerVerification;

impl rustls::client::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}
