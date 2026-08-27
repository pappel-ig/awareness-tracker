mod db;
mod mail;
mod routes;
mod tls;

use std::env;
use std::sync::Arc;

use anyhow::Result;
use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::Extension;
use dotenvy::dotenv;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use hyper_util::service::TowerToHyperService;
use tokio::net::TcpListener;
use tower::Service;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .map_err(|_| anyhow::anyhow!("Crypto-Provider konnte nicht installiert werden"))?;

    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8443".to_string());
    let database_url = env::var("DATABASE_URL")?;
    let server_config = Arc::new(tls::load_server_config()?);
    let db = Arc::new(db::connect(&database_url).await?);
    let listener = TcpListener::bind(&bind_addr).await?;
    let app = routes::router().layer(Extension(db));

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
