
use std::env;
use std::io;
use std::sync::Arc;

use anyhow::{Context, Result};
use rustls::server::Acceptor;
use rustls::ServerConfig;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_rustls::LazyConfigAcceptor;

#[derive(Debug, Clone, Serialize)]
pub struct ClientHelloInfo {
    pub offered_cipher_suites: Vec<String>,
    pub negotiated_cipher_suite: Option<String>,
    pub server_name: Option<String>,
    pub alpn_protocols: Vec<String>,
}

fn cipher_suite_name(suite: rustls::CipherSuite) -> String {
    format!("{suite:?}")
}

pub async fn accept_tls(
    tcp: TcpStream,
    config: Arc<ServerConfig>,
) -> Result<(tokio_rustls::server::TlsStream<TcpStream>, ClientHelloInfo)> {
    let acceptor = LazyConfigAcceptor::new(Acceptor::default(), tcp);
    tokio::pin!(acceptor);

    let start = acceptor
        .as_mut()
        .await
        .context("TLS-ClientHello konnte nicht gelesen werden")?;

    let hello = start.client_hello();
    let mut info = ClientHelloInfo {
        offered_cipher_suites: hello
            .cipher_suites()
            .iter()
            .map(|s| cipher_suite_name(*s))
            .collect(),
        negotiated_cipher_suite: None,
        server_name: hello.server_name().map(str::to_string),
        alpn_protocols: hello
            .alpn()
            .map(|protocols| {
                protocols
                    .map(|p| String::from_utf8_lossy(p).into_owned())
                    .collect()
            })
            .unwrap_or_default(),
    };

    let stream = start
        .into_stream(config)
        .await
        .context("TLS-Handshake fehlgeschlagen")?;

    info.negotiated_cipher_suite = stream
        .get_ref()
        .1
        .negotiated_cipher_suite()
        .map(|s| cipher_suite_name(s.suite()));

    Ok((stream, info))
}

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
