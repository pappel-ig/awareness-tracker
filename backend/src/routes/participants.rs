use std::collections::HashMap;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use email_address::EmailAddress;
use tokio_postgres::Client;
use uuid::Uuid;
use crate::Config;
use crate::mail::EmailTemplateService;

pub fn router() -> Router {
    Router::new()
        .route("/participants", post(new_participant))
}

#[derive(Deserialize)]
pub struct NewParticipantRequest {
    pub email: String,
}

async fn new_participant(
    Extension(db): Extension<Arc<Client>>,
    Extension(email): Extension<EmailTemplateService>,
    Extension(config): Extension<Config>,
    Json(body): Json<NewParticipantRequest>,
) -> Result<Json<Value>, StatusCode> {
    if body.email.trim().is_empty() || !EmailAddress::is_valid(&body.email) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let uuid = Uuid::new_v4();
    let mut values = HashMap::new();
    println!("{}", uuid.to_string());
    values.insert("id", uuid.to_string());
    values.insert("bind", config.addr);
    email.send_template(
        "templates/invite.html",
        "Umfrage Security Awareness",
        "tbd",
        body.email.trim(),
        values,
    ).map_err(|e| {
        println!("Failed to send template: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    db.execute(
        "INSERT INTO participants (id, email) VALUES ($1, $2)",
        &[&uuid, &body.email],
    ).await
        .map_err(|e| {
            println!("Failed to insert participant: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(json!({
        "status": "ok"
    })))
}