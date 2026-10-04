use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use axum::middleware;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::Arc;
use email_address::EmailAddress;
use log::warn;
use middleware::from_fn;
use tokio_postgres::Client;
use uuid::Uuid;

use crate::auth::{generate_token, hash_token, require_participant, Participant};
use crate::mail::EmailTemplateService;
use crate::turnstile::TurnstileClient;
use crate::Config;

pub fn router() -> Router {
    Router::new()
        .route("/participants", post(new_participant))
        .route("/participants/leaks", post(update_leak_status).route_layer(from_fn(require_participant)))
        .route("/participants", get(me).route_layer(from_fn(require_participant)))
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
    Extension(mailer): Extension<EmailTemplateService>,
    Extension(config): Extension<Config>,
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
    let token = generate_token();
    let token_hash = hash_token(&token);

    let inserted = db.query_opt(
        "INSERT INTO participants (id, email, token_hash, leak_check, invite_sent) VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (email) DO NOTHING
         RETURNING id",
        &[&uuid, &body.email, &token_hash, &body.leak_check, &!body.leak_check],
    ).await
        .map_err(|e| {
            warn!("Failed to insert participant: {}", e.as_db_error().expect("db error:"));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    if inserted.is_some() && !body.leak_check {
        let email = body.email.clone();
        tokio::spawn(async move {
            let result = tokio::task::spawn_blocking(move || {
                mailer
                    .send_invite(&config, &email, &token)
                    .map_err(|e| format!("{e:#}"))
            })
                .await;
            if let Ok(Err(e)) = result {
                warn!("Failed to send invite mail: {}", e);
            }
        });
    }

    Ok(Json(json!({
        "status": "ok"
    })))
}

async fn update_leak_status(
    Extension(db): Extension<Arc<Client>>,
    Extension(participant): Extension<Participant>
) -> Result<StatusCode, StatusCode> {
    db
        .execute(
            "UPDATE participants SET leak_check = true WHERE id = $1",
            &[&participant.id],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::ACCEPTED)
}

async fn me(
    Extension(db): Extension<Arc<Client>>,
    Extension(participant): Extension<Participant>
) -> Result<Json<Value>, StatusCode> {

    let row = db.query_one(
        "SELECT survey_sent FROM participants WHERE id = $1",
        &[&participant.id]
    ).await
        .map_err(|e| {
            warn!("Failed to insert participant: {}", e.as_db_error().expect("db error:"));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let survey_sent: bool = row.get("survey_sent");
    Ok(Json(json!({
        "survey_sent": survey_sent,
    })))
}