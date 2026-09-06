use axum::extract::ConnectInfo;
use axum::http::StatusCode;
use axum::middleware;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
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
        .route("/participants/me", get(me).route_layer(from_fn(require_participant)))
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
        "INSERT INTO participants (id, email, token_hash, leak_check) VALUES ($1, $2, $3, $4)
         ON CONFLICT (email) DO NOTHING
         RETURNING id",
        &[&uuid, &body.email, &token_hash, &body.leak_check],
    ).await
        .map_err(|e| {
            warn!("Failed to insert participant: {}", e.as_db_error().expect("db error:"));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if inserted.is_some() {
        let email = body.email.clone();
        let survey_url = format!("https://{}/survey?token={}", config.frontend_addr, token);
        let mut vals = HashMap::new();
        vals.insert("survey", survey_url);
        vals.insert("bind", config.addr.clone());
        vals.insert("token", token);

        tokio::spawn(async move {
            let result = tokio::task::spawn_blocking(move || {
                mailer
                    .send_template(
                        "templates/invite.html",
                        "Deine Einladung zur Security Awareness Umfrage",
                        &email,
                        vals,
                    )
                    .map_err(|e| e.to_string())
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

async fn me(Extension(participant): Extension<Participant>) -> Json<Value> {
    Json(json!({
        "id": participant.id,
    }))
}