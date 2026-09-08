use crate::auth::{Participant, require_participant};
use crate::ip::IpInformationService;
use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use axum::middleware::from_fn;
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde_json::{Value, json};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_postgres::Client;

pub fn router() -> Router {
    Router::new()
        .route("/information/leaks", get(leak_status).route_layer(from_fn(require_participant)))
        .route("/information/ip", get(ip).route_layer(from_fn(require_participant)))
}

async fn leak_status(
    Extension(participant): Extension<Participant>,
    Extension(db): Extension<Arc<Client>>,
) -> Result<Json<Value>, StatusCode> {
    let row = db
        .query_opt(
            "SELECT leak_check, leak_breaches FROM participants WHERE id = $1",
            &[&participant.id],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let leak_check: Option<bool> = row.get("leak_check");
    let leak_breaches: Option<Value> = row.get("leak_breaches");

    Ok(Json(json!({
        "leak_check": leak_check.unwrap_or(false),
        "breaches": leak_breaches.unwrap_or_else(|| json!([])),
    })))
}

async fn ip(
    Extension(ConnectInfo(remote_addr)): Extension<ConnectInfo<SocketAddr>>,
    Extension(ip_client): Extension<Arc<IpInformationService>>,
) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!(ip_client.lookup(remote_addr.ip()))))
}

