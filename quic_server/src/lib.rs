pub fn make_self_signed_cert(
) -> Result<(rustls::Certificate, rustls::PrivateKey), Box<dyn std::error::Error>> {
    let certified_key = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;

    let cert_der = certified_key.cert.der().to_vec();
    let key_der = certified_key.key_pair.serialize_der();

    let key = rustls::PrivateKey(key_der);
    let cert = rustls::Certificate(cert_der);

    Ok((cert, key))
}
