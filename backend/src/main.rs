mod auth;
mod db;
mod leak_api;
mod mail;
mod routes;
mod tls;
mod turnstile;
mod ip;

use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::Extension;
use axum::middleware::from_fn;
use axum_server::tls_rustls::RustlsConfig;
use dotenvy::dotenv;
use hyper::http::HeaderValue;
use log::{info, warn};
use tower_http::cors::{Any, CorsLayer};
use ip::IpInformationService;
use crate::mail::EmailTemplateService;
use crate::routes::router;

#[derive(Clone)]
pub struct Config {
    pub addr: String,
    pub frontend_addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stdout)
        .init();
    match dotenv() {
        Err(e) if !e.not_found() => warn!(".env konnte nicht geladen werden: {e}"),
        _ => {}
    }
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .map_err(|_| anyhow::anyhow!("Crypto-Provider konnte nicht installiert werden"))?;

    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let addr = env::var("ADDR").unwrap_or_else(|_| "localhost:8080".to_string());
    let frontend_addr = env::var("FRONTEND_ADDR").unwrap_or_else(|_| "localhost:3000".to_string());
    let cors = env::var("CORS").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://tracker:tracker@127.0.0.1:5432/tracker".to_string());
    let server_config = Arc::new(tls::load_server_config()?);
    let db = Arc::new(db::connect(&database_url).await?);
    let ip_information_client = IpInformationService::new()?;
    let leak_client = leak_api::build_client()?;
    let turnstile_client = Arc::new(turnstile::TurnstileClient::new()?);
    let email_template_service = EmailTemplateService::new()?;
    let config = Config { addr, frontend_addr };
    tokio::spawn(leak_api::run_worker(db.clone(), leak_client.clone(), email_template_service.clone(), config.clone()));

    info!("Start Backend on {}", bind_addr);

    let cors = CorsLayer::new()
        .allow_origin(cors.parse::<HeaderValue>()?)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = router()
        .layer(cors)
        .layer(Extension(config))
        .layer(Extension(db))
        .layer(Extension(leak_client))
        .layer(Extension(turnstile_client))
        .layer(Extension(email_template_service))
        .layer(Extension(Arc::new(ip_information_client)))
        .layer(from_fn(ip::real_ip));

    tokio::spawn(async {
        shutdown_signal().await;
        info!("Stopping Backend");
        std::process::exit(0);
    });

    info!("Started Backend on {}", bind_addr);

    axum_server::bind_rustls(bind_addr.parse::<SocketAddr>()?, RustlsConfig::from_config(server_config))
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .context("Serve failed")?;

    Ok(())
}

#[cfg(unix)]
async fn shutdown_signal() {
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to install SIGTERM handler");
    tokio::select! {
        _ = sigterm.recv() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
}

#[cfg(not(unix))]
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
}
