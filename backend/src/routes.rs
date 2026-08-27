use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{ConnectInfo, Extension, Path};
use axum::http::{HeaderMap, Method, StatusCode, Uri};
use axum::response::{Json, Redirect};
use axum::routing::get;
use axum::Router;
use serde_json::{json, Value};
use tokio_postgres::Client;
use uuid::Uuid;

use crate::tls::ClientHelloInfo;

pub fn router() -> Router {
    Router::new()
        .route("/tls-info", get(tls_info))
        .route("/track", get(track_redirect))
        .route("/track/{id}", get(track))
}

async fn tls_info(Extension(info): Extension<Arc<ClientHelloInfo>>) -> Json<ClientHelloInfo> {
    Json((*info).clone())
}

async fn track_redirect() -> Redirect {
    Redirect::to(&format!("/track/{}", Uuid::new_v4()))
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
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "id": id,
        "remote_addr": remote_addr,
        "tls": tls,
        "headers": headers,
    })))
}
