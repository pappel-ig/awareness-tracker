
use std::env;
use std::io;

use anyhow::{Context, Result};
use rustls::ServerConfig;

pub fn load_server_config() -> Result<ServerConfig> {
    let cert_path = env::var("TLS_CERT_PATH").ok();
    let key_path = env::var("TLS_KEY_PATH").ok();

    let (certs, key) = match (cert_path, key_path) {
        (Some(cert_path), Some(key_path)) => load_cert_and_key(&cert_path, &key_path)?,
        _ => generate_self_signed()?,
    };

    let mut config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .context("TLS-Serverkonfiguration konnte nicht erstellt werden")?;
    config.alpn_protocols = vec![b"http/1.1".to_vec()];

    Ok(config)
}

fn load_cert_and_key(
    cert_path: &str,
    key_path: &str,
) -> Result<(
    Vec<rustls::pki_types::CertificateDer<'static>>,
    rustls::pki_types::PrivateKeyDer<'static>,
)> {
    let cert_bytes =
        std::fs::read(cert_path).with_context(|| format!("Zertifikat {cert_path} nicht lesbar"))?;
    let key_bytes =
        std::fs::read(key_path).with_context(|| format!("Private Key {key_path} nicht lesbar"))?;

    let certs = rustls_pemfile::certs(&mut &cert_bytes[..])
        .collect::<io::Result<Vec<_>>>()
        .context("Zertifikat konnte nicht geparst werden")?;
    let key = rustls_pemfile::private_key(&mut &key_bytes[..])
        .context("Private Key konnte nicht geparst werden")?
        .context("Keine Private-Key-PEM-Sektion gefunden")?;

    Ok((certs, key))
}

fn generate_self_signed() -> Result<(
    Vec<rustls::pki_types::CertificateDer<'static>>,
    rustls::pki_types::PrivateKeyDer<'static>,
)> {
    let generated = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])
        .context("Selbstsigniertes Zertifikat konnte nicht generiert werden")?;

    let cert_der = rustls::pki_types::CertificateDer::from(generated.cert);
    let key_der =
        rustls::pki_types::PrivateKeyDer::try_from(generated.signing_key.serialize_der())
            .map_err(|e| anyhow::anyhow!("Private Key konnte nicht konvertiert werden: {e}"))?;

    Ok((vec![cert_der], key_der))
}
