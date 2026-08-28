use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::body::Bytes;
use axum::extract::{ConnectInfo, Extension, Path};
use axum::http::{HeaderMap, Method, StatusCode, Uri, header};
use axum::response::IntoResponse;
use axum::routing::get;
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
        .route("/tracker/mail/{id}", get(tracker_pixel))
}

async fn tracker_pixel(
        Path(id): Path<Uuid>,
        method: Method,
        uri: Uri,
        headers: HeaderMap,
        Extension(tls_info): Extension<Arc<ClientHelloInfo>>,
        Extension(ConnectInfo(remote_addr)): Extension<ConnectInfo<SocketAddr>>,
        Extension(db): Extension<Arc<Client>>,
) -> Result<impl IntoResponse, StatusCode> {

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

    Ok(([(header::CONTENT_TYPE, "image/gif")], Bytes::from_static(PIXEL_GIF)))
}
