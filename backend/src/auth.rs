use std::sync::Arc;

use axum::Extension;
use axum::extract::{Query, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::Rng;
use rand::rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio_postgres::Client;
use uuid::Uuid;

const TOKEN_BYTES: usize = 32;

#[derive(Clone, Debug)]
pub struct Participant {
    pub id: Uuid,
}

pub fn generate_token() -> String {
    let mut bytes = [0u8; TOKEN_BYTES];
    rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn hash_token(token: &str) -> Vec<u8> {
    Sha256::digest(token.as_bytes()).to_vec()
}

#[derive(Deserialize)]
struct TokenQuery {
    token: Option<String>,
}

pub async fn require_participant(
    Extension(db): Extension<Arc<Client>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token_params = Query::<TokenQuery>::try_from_uri(request.uri()).map_err(|_| StatusCode::UNAUTHORIZED);
    let token = token_params.unwrap().0.token.filter(|t| !t.is_empty()).ok_or(StatusCode::UNAUTHORIZED)?;
    let token_hash = hash_token(&token);

    let row = db
        .query_opt(
            "SELECT id, email FROM participants WHERE token_hash = $1",
            &[&token_hash],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(Participant {
        id: row.get("id"),
    });

    Ok(next.run(request).await)
}
