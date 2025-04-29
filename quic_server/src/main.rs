use anyhow::Result;
use quinn::{Endpoint, ServerConfig};
use std::net::SocketAddr;
use tracing::{error, info};
use tracing_subscriber;

const PORT: u16 = 4433;
const ADDRESS: &str = "127.0.0.1";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (cert, key) = quic_server::make_self_signed_cert().unwrap();
    let server_config = ServerConfig::with_single_cert(vec![cert], key).unwrap();

    let addr = format!("{}:{}", ADDRESS, PORT)
        .parse::<SocketAddr>()
        .unwrap();

    let endpoint = Endpoint::server(server_config, addr).unwrap();

    info!("QUIC server listening on {}", addr);

    while let Some(conn) = endpoint.accept().await {
        tokio::spawn(handle_connection(conn));
    }
}

async fn handle_connection(conn: quinn::Connecting) -> Result<()> {
    let connection = match conn.await {
        Ok(c) => c,
        Err(e) => {
            error!("Connection failed: {}", e);
            return Ok(());
        }
    };

    info!("New connection established!{}", connection.remote_address());

    loop {
        let stream_result = connection.accept_bi().await;
        match stream_result {
            Err(quinn::ConnectionError::ApplicationClosed { .. }) => {
                info!("Connection closed!");
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
            Ok((mut send, mut recv)) => {
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 1024];

                    match recv.read(&mut buf).await {
                        Ok(n) => {
                            if n == Some(0) {
                                return;
                            }
                            let received = &buf[..n.unwrap()];
                            info!("Received: {:?}", String::from_utf8_lossy(received));

                            // Echo back
                            if let Err(e) = send.write_all(b"Hello from server").await {
                                error!("Failed to send: {e}");
                            }
                        }
                        Err(e) => {
                            error!("Error reading from stream: {}", e);
                            info!("Client connection closed due to error");
                            return;
                        }
                    }
                });
            }
        }
    }
}
