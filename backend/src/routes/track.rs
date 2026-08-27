use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::{ConnectInfo, Extension, Path};
use axum::http::{header, HeaderMap, Method, StatusCode, Uri};
use axum::response::{IntoResponse, Json};
use axum::routing::get;
use axum::Router;
use serde_json::{json, Value};
use tokio_postgres::Client;
use uuid::Uuid;

use crate::tls::ClientHelloInfo;

const PIXEL_GIF: &[u8] = &[
    0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x01, 0x00, 0x01, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xFF, 0xFF, 0xFF, 0x21, 0xF9, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x2C, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x02, 0x44, 0x01, 0x00, 0x3B,
];

pub fn router() -> Router {
    Router::new()
        .route("/track/{id}", get(track))
        .route("/tracker/mail/{id}", get(tracker_pixel))
}

async fn tracker_pixel(Path(_id): Path<Uuid>) -> impl IntoResponse {
    println!("TEST INFO");
    ([(header::CONTENT_TYPE, "image/gif")], Bytes::from_static(PIXEL_GIF))
}

async fn track(
    Path(id): Path<Uuid>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    Extension(tls_info): Extension<Arc<ClientHelloInfo>>,
    Extension(ConnectInfo(remote_addr)): Extension<ConnectInfo<SocketAddr>>,
    Extension(db): Extension<Arc<Client>>,
) -> Result<Json<Value>, StatusCode> {
    let headers: BTreeMap<String, String> = headers
        .iter()
        .map(|(name, value)| {
            (
                name.to_string(),
                String::from_utf8_lossy(value.as_bytes()).into_owned(),
            )
        })
        .collect();

    let tls = serde_json::to_value(&*tls_info).unwrap();
    let headers = serde_json::to_value(&headers).unwrap();
    let uri = uri.to_string();
    let remote_addr = remote_addr.to_string();
    let method = method.as_str();

    db.execute(
        "INSERT INTO tracks (id, method, uri, remote_addr, tls, headers) VALUES ($1, $2, $3, $4, $5, $6)",
        &[&id, &method, &uri, &remote_addr, &tls, &headers],
    ).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "id": id,
        "remote_addr": remote_addr,
        "tls": tls,
        "headers": headers,
    })))
}
