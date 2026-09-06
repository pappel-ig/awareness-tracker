use std::collections::BTreeMap;
use std::fmt::format;
use std::net::SocketAddr;
use std::string::ToString;
use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::body::Bytes;
use axum::extract::{ConnectInfo, Extension};
use axum::http::{HeaderMap, Method, StatusCode, Uri, header};
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::get;
use log::{error, warn};
use middleware::from_fn;
use serde_json::{Value, json};
use tokio_postgres::Client;
use uuid::Uuid;

use crate::auth::{Participant, require_participant};
use crate::tls::ClientHelloInfo;

const PIXEL_GIF: &[u8] = &[
    0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x01, 0x00, 0x01, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x21, 0xF9, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x2C, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x02, 0x44, 0x01, 0x00, 0x3B,
];

pub fn router() -> Router {
    Router::new()
        .route("/browser/tracker", get(tracker_browser).route_layer(from_fn(require_participant)))
        .route("/mail/tracker", get(tracker_pixel).route_layer(from_fn(require_participant)))
        .route("/leaks", get(leak_status).route_layer(from_fn(require_participant)))
}

async fn tracker_browser(
    method: Method,
    headers: HeaderMap,
    Extension(tls_info): Extension<Arc<ClientHelloInfo>>,
    Extension(ConnectInfo(remote_addr)): Extension<ConnectInfo<SocketAddr>>,
    Extension(db): Extension<Arc<Client>>,
    Extension(participant): Extension<Participant>,
) -> Result<Json<Value>, StatusCode> {
    let headers = extract_headers(headers);
    let tls = serde_json::to_value(&*tls_info).unwrap();
    let headers = serde_json::to_value(&headers).unwrap();
    let remote_addr = remote_addr.to_string();
    let method = method.as_str();

    db.execute(
        "INSERT INTO tracks (id, participant, method, origin, remote_addr, tls, headers) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&Uuid::new_v4(), &participant.id, &method, &"browser", &remote_addr, &tls, &headers],
    ).await
        .map_err(|e| {
            error!("Unable to insert track data: {}", e.as_db_error().expect("db error"));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(json!("ok")))
}

fn extract_headers(headers: HeaderMap) -> BTreeMap<String, String> {
    let headers: BTreeMap<String, String> = headers
        .iter()
        .map(|(name, value)| {
            (
                name.to_string(),
                String::from_utf8_lossy(value.as_bytes()).into_owned(),
            )
        })
        .collect();
    headers
}

async fn tracker_pixel(
        method: Method,
        headers: HeaderMap,
        Extension(tls_info): Extension<Arc<ClientHelloInfo>>,
        Extension(ConnectInfo(remote_addr)): Extension<ConnectInfo<SocketAddr>>,
        Extension(db): Extension<Arc<Client>>,
        Extension(participant): Extension<Participant>,
) -> Result<impl IntoResponse, StatusCode> {
    let headers = extract_headers(headers);
    let tls = serde_json::to_value(&*tls_info).unwrap();
    let headers = serde_json::to_value(&headers).unwrap();
    let remote_addr = remote_addr.to_string();
    let method = method.as_str();

    db.execute(
        "INSERT INTO tracks (id, participant, method, origin, remote_addr, tls, headers) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&Uuid::new_v4(), &participant.id, &method, &"mail", &remote_addr, &tls, &headers],
    ).await
        .map_err(|e| {
            error!("Unable to insert track data: {}", e.as_db_error().expect("db error"));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(([(header::CONTENT_TYPE, "image/gif")], Bytes::from_static(PIXEL_GIF)))
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
        "checked": leak_breaches.is_some(),
        "breaches": leak_breaches.unwrap_or_else(|| json!([])),
    })))
}
