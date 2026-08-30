use axum::http::StatusCode;
use axum::routing::post;
use axum::{Extension, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use email_address::EmailAddress;
use tokio_postgres::Client;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/participants", post(new_participant))
}

#[derive(Deserialize)]
pub struct NewParticipantRequest {
    pub email: String,
    pub leak_check: bool
}

async fn new_participant(
    Extension(db): Extension<Arc<Client>>,
    Json(body): Json<NewParticipantRequest>,
) -> Result<Json<Value>, StatusCode> {
    if body.email.trim().is_empty() || !EmailAddress::is_valid(&body.email) {
        return Err(StatusCode::BAD_REQUEST);
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