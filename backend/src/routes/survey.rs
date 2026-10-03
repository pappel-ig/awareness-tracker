use crate::auth::{require_participant};
use crate::ip::IpInformation;
use axum::http::StatusCode;
use axum::middleware::from_fn;
use axum::routing::post;
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;
use log::{error, warn};
use tokio_postgres::Client;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/survey", post(submit).route_layer(from_fn(require_participant)))
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SurveySubmitRequest {
    pub meta: SurveyMeta,
    pub age: AgeGroup,
    pub it_knowledge: Rating,
    pub security_awareness: Rating,
    pub leak_knowledge: bool,
    pub leak_scare_factor: Rating,
    pub ip_knowledge: bool,
    pub ip_scare_factor: Rating,
    pub fingerprint_knowledge: bool,
    pub fingerprint_scare_factor: Rating,
    pub survey_behavior: SurveyBehavior,
    pub tracking_pixel_knowledge: bool,
    pub tracking_pixel_scare_factor: Rating,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(try_from = "u8")]
pub struct Rating(u8);

impl TryFrom<u8> for Rating {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1..=5 => Ok(Rating(value)),
            _ => Err("Ordinale Typen erlauben nur Werte im Bereich von 1-5".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AgeGroup {
    #[serde(rename = "Keine Angabe")]
    NoAnswer,
    #[serde(rename = "0-18")]
    Under18,
    #[serde(rename = "18-25")]
    From18To25,
    #[serde(rename = "26-45")]
    From26To45,
    #[serde(rename = "46- 55")]
    From46To55,
    #[serde(rename = "55+")]
    Over55,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SurveyMeta {
    pub ip: Option<IpInformation>,
    pub leak: Option<LeakMeta>,
    pub tracker: Option<Vec<TrackerMeta>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeakMeta {
    pub breaches: Vec<BreachMeta>,
    pub leak_check: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BreachMeta {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Domain")]
    pub domain: String,
    #[serde(rename = "DataClasses")]
    pub data_classes: Vec<String>,
    #[serde(rename = "AddedDate")]
    pub added_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackerMeta {
    pub created_at: String,
    pub remote_addr: String,
    pub remote_addr_info: Option<IpInformation>,
    pub headers: Value,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SurveyBehavior {
    pub vpn: bool,
    pub password_reuse: bool,
    pub password_change: bool,
    pub password_manager: bool,
    pub passkeys: bool,
    pub two_factor: bool,
    pub updates: bool,
    pub public_wifi: bool,
    pub ad_blocker: bool,
}

async fn submit(
    Extension(db): Extension<Arc<Client>>,
    Json(body): Json<SurveySubmitRequest>,
) -> Result<Json<Value>, StatusCode> {
    let uuid = Uuid::new_v4();
    db.query(
        "INSERT INTO surveys (id, data) VALUES ($1, $2) RETURNING id",
        &[&uuid, &json!(body)],
    ).await
        .map_err(|e| {
            error!("Failed to insert survey: {}", e.as_db_error().expect("db error:"));
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(json!({ "status": "ok"})))
}