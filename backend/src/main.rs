mod db;
mod mail;
mod routes;
mod tls;

use std::env;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::Extension;
use dotenvy::dotenv;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use hyper_util::service::TowerToHyperService;
use tokio::net::TcpListener;
use tower::Service;
use log::info;
use crate::mail::EmailTemplateService;
use crate::routes::router;

#[derive(Clone)]
pub struct Config {
    pub addr: String
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .map_err(|_| anyhow::anyhow!("Crypto-Provider konnte nicht installiert werden"))?;

    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8443".to_string());
    let addr = env::var("ADDR").unwrap_or_else(|_| "localhost:8443".to_string());
    let database_url = env::var("DATABASE_URL")?;
    let server_config = Arc::new(tls::load_server_config()?);
    let db = Arc::new(db::connect(&database_url).await?);
    let email_template_service = EmailTemplateService::new(
        env::var("SMTP_SERVER")?,
        env::var("SMTP_PORT")?.parse().context("SMTP_PORT invalid value")?,
        env::var("SMTP_USERNAME")?,
        env::var("SMTP_PASSWORD")?,
        env::var("SMTP_NAME")?,
        env::var("SMTP_FROM")?
    );

    info!("Start Backend on {}", bind_addr);

    let listener = TcpListener::bind(&bind_addr).await?;
    let app = router()
        .layer(Extension(Config {addr}))
        .layer(Extension(db))
        .layer(Extension(email_template_service));

    tokio::spawn(async {
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler");
        tokio::select! {
            _ = sigterm.recv() => {},
            _ = tokio::signal::ctrl_c() => {},
        }
        info!("Stopping Backend");
        std::process::exit(0);
    });

    info!("Started Backend on {}", bind_addr);

    loop {
        let Ok((tcp_stream, peer_addr)) = listener.accept().await else {
            continue;
        };
        let server_config = server_config.clone();
        let app = app.clone();

        tokio::spawn(async move {
            let Ok((tls_stream, hello_info)) = tls::accept_tls(tcp_stream, server_config).await
            else {
                return;
            };

            let mut app = app
                .layer(Extension(Arc::new(hello_info)))
                .layer(Extension(ConnectInfo(peer_addr)));
            let hyper_service = TowerToHyperService::new(tower::service_fn(
                move |request: hyper::Request<hyper::body::Incoming>| {
                    app.call(request.map(Body::new))
                },
            ));

            let _ = Builder::new(TokioExecutor::new())
                .serve_connection(TokioIo::new(tls_stream), hyper_service)
                .await;
        });
    }
}
