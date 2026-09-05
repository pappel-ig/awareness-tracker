use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use email_address::EmailAddress;
use log::warn;
use tokio_postgres::Client;
use uuid::Uuid;

use crate::turnstile::TurnstileClient;

pub fn router() -> Router {
    Router::new()
        .route("/participants", post(new_participant))
}

#[derive(Deserialize)]
pub struct NewParticipantRequest {
    pub email: String,
    pub leak_check: bool,
    pub turnstile_token: String,
}

async fn new_participant(
    Extension(db): Extension<Arc<Client>>,
    Extension(turnstile): Extension<Arc<TurnstileClient>>,
    connect_info: Option<Extension<ConnectInfo<SocketAddr>>>,
    Json(body): Json<NewParticipantRequest>,
) -> Result<Json<Value>, StatusCode> {
    if body.email.trim().is_empty() || !EmailAddress::is_valid(&body.email) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let remote_ip = connect_info.map(|Extension(ConnectInfo(addr))| addr.ip().to_string());
    let verified = turnstile
        .verify(&body.turnstile_token, remote_ip.as_deref())
        .await
        .map_err(|e| {
            warn!("turnstile verification request failed: {}", e);
            StatusCode::BAD_GATEWAY
        })?;
    if !verified {
        return Err(StatusCode::FORBIDDEN);
    }

    let uuid = Uuid::new_v4();
    db.execute(
        "INSERT INTO participants (id, email, leak_check) VALUES ($1, $2, $3) ON CONFLICT (email) DO NOTHING",
        &[&uuid, &body.email, &body.leak_check],
    ).await
        .map_err(|e| {
            println!("Failed to insert participant: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(json!({
        "status": "ok"
    })))
}