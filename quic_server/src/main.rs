use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::Arc;

fn make_self_signed_cert(
) -> Result<(rustls::Certificate, rustls::PrivateKey), Box<dyn std::error::Error>> {
    let certified_key = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;

    let cert_der = certified_key.cert.der().to_vec();
    let key_der = certified_key.key_pair.serialize_der();

    let key = rustls::PrivateKey(key_der);
    let cert = rustls::Certificate(cert_der);

    Ok((cert, key))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate self-signed certificate
    let (cert, key) = make_self_signed_cert()?;
    println!("Generated self-signed certificate!");

    // Set up server config
    let server_config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(vec![cert.clone()], key)?;
    let server_config = Arc::new(server_config);

    // Start listening
    let listener = TcpListener::bind("127.0.0.1:8443")?;
    println!("Server listening on 127.0.0.1:8443");

    for stream in listener.incoming() {
        match stream {
            Ok(tcp_stream) => {
                let server_config = server_config.clone();

                std::thread::spawn(move || {
                    let mut tls_stream = rustls::ServerConnection::new(server_config)
                        .map(|conn| rustls::StreamOwned::new(conn, tcp_stream))
                        .expect("Failed to create TLS stream");

                    println!("Accepted new TLS connection!");

                    let mut buffer = [0u8; 1024];
                    match tls_stream.read(&mut buffer) {
                        Ok(count) => {
                            println!("Read {} bytes from client", count);
                            if count > 0 {
                                match std::str::from_utf8(&buffer[..count]) {
                                    Ok(text) => println!("Client said: {}", text),
                                    Err(_) => println!(
                                        "Client sent non-UTF8 data: {:?}",
                                        &buffer[..count]
                                    ),
                                }
                                let _ = tls_stream.write_all(&buffer[..count]);
                            }
                        }
                        Err(e) => {
                            println!("Error reading from TLS stream: {:?}", e);
                        }
                    }
                });
            }
            Err(e) => {
                println!("Connection failed: {:?}", e);
            }
        }
    }

    Ok(())
}
