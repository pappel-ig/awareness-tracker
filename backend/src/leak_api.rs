use std::env;
use std::sync::Arc;
use std::time::Duration;
use anyhow::anyhow;
use log::{error, warn};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::serde::rfc3339;
use time::{Date, OffsetDateTime};
use tokio_postgres::Client as DbClient;
use uuid::Uuid;

use crate::auth::{generate_token, hash_token};
use crate::mail::EmailTemplateService;
use crate::Config;

const POLL_INTERVAL: Duration = Duration::from_secs(5);
const MAX_BACKOFF: Duration = Duration::from_secs(30 * 60);
const DEFAULT_RATE_LIMIT_WAIT: Duration = Duration::from_secs(10);

const HIBP_API_URL: &str = "https://haveibeenpwned.com/api/v3/breachedaccount";

#[derive(Deserialize, Serialize)]
struct Breach {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Domain")]
    domain: String,
    #[serde(rename = "DataClasses")]
    data: Vec<String>,
    #[serde(rename = "AddedDate",  with = "rfc3339")]
    ts: OffsetDateTime
}

enum CheckError {
    RateLimited(Duration),
    Other(anyhow::Error),
}

impl<E: Into<anyhow::Error>> From<E> for CheckError {
    fn from(e: E) -> Self {
        CheckError::Other(e.into())
    }
}

pub fn build_client() -> anyhow::Result<Arc<Client>> {
    let api_key = env::var("HIBP_API_KEY")
        .map_err(|_| anyhow!("HIBP_API_KEY environment variable is not set"))?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("hibp-api-key", api_key.parse()?);
    headers.insert(reqwest::header::USER_AGENT, "awareness-tracker".parse()?);

    let client = Client::builder().default_headers(headers).build()?;
    Ok(Arc::new(client))
}

async fn check_email(client: &Client, email: &str) -> Result<Vec<Breach>, CheckError> {
    let mut url = reqwest::Url::parse(HIBP_API_URL)?;
    url.path_segments_mut()
        .map_err(|_| anyhow!("HIBP_API_URL is not a valid base URL"))?
        .push(email);
    url.query_pairs_mut().append_pair("truncateResponse", "false");

    let response = client.get(url).send().await?;

    match response.status() {
        StatusCode::OK => {
            Ok(response.json().await?)
        }
        StatusCode::NOT_FOUND => Ok(Vec::new()),
        StatusCode::TOO_MANY_REQUESTS => {
            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .map(Duration::from_secs)
                .unwrap_or(DEFAULT_RATE_LIMIT_WAIT);
            Err(CheckError::RateLimited(retry_after))
        }
        status => Err(CheckError::Other(anyhow!(
            "unexpected status {status}"
        ))),
    }
}

pub async fn run_worker(db: Arc<DbClient>, client: Arc<Client>, mailer: EmailTemplateService, config: Config) {
    let mut backoff = POLL_INTERVAL;

    loop {
        send_pending_invites(&db, &mailer, &config).await;

        let pending = db
            .query_opt(
                "SELECT id, email FROM participants
                 WHERE leak_check AND leak_breaches IS NULL
                 ORDER BY registered_at LIMIT 1",
                &[],
            )
            .await;

        let row = match pending {
            Ok(Some(row)) => row,
            Ok(None) => {
                tokio::time::sleep(POLL_INTERVAL).await;
                continue;
            }
            Err(e) => {
                error!("leak check worker: failed to query pending participants: {e}");
                tokio::time::sleep(POLL_INTERVAL).await;
                continue;
            }
        };

        let id: Uuid = row.get("id");
        let email: String = row.get("email");

        match check_email(&client, &email).await {
            Ok(breaches) => {
                backoff = POLL_INTERVAL;
                let breaches = json!(breaches);
                if let Err(e) = db
                    .execute(
                        "UPDATE participants SET leak_breaches = $1 WHERE id = $2",
                        &[&breaches, &id],
                    )
                    .await
                {
                    error!("leak check worker: failed to store result for {id}: {e}");
                }
            }
            Err(CheckError::RateLimited(retry_after)) => {
                warn!("leak check worker: rate limited by haveibeenpwned.com, waiting {retry_after:?}"
                );
                tokio::time::sleep(retry_after).await;
            }
            Err(CheckError::Other(e)) => {
                error!("leak check worker: check failed for {id}, backing off {backoff:?}: {e}");
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
            }
        }
    }
}

async fn send_pending_invites(db: &DbClient, mailer: &EmailTemplateService, config: &Config) {
    let rows = match db
        .query(
            "SELECT id, email FROM participants
             WHERE NOT invite_sent AND leak_breaches IS NOT NULL
             ORDER BY registered_at",
            &[],
        )
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            error!("leak check worker: failed to query pending invites: {e}");
            return;
        }
    };

    for row in rows {
        let id: Uuid = row.get("id");
        let email: String = row.get("email");

        let token = generate_token();
        let token_hash = hash_token(&token);
        if let Err(e) = db
            .execute("UPDATE participants SET token_hash = $1 WHERE id = $2", &[&token_hash, &id])
            .await
        {
            error!("leak check worker: failed to store token for {id}: {e}");
            continue;
        }

        let mailer = mailer.clone();
        let config = config.clone();
        let sent = tokio::task::spawn_blocking(move || mailer.send_invite(&config, &email, &token)).await;

        match sent {
            Ok(Ok(())) => {
                if let Err(e) = db
                    .execute("UPDATE participants SET invite_sent = TRUE WHERE id = $1", &[&id])
                    .await
                {
                    error!("leak check worker: failed to mark invite as sent for {id}: {e}");
                }
            }
            Ok(Err(e)) => warn!("leak check worker: failed to send invite for {id}: {e:#}"),
            Err(e) => error!("leak check worker: invite task for {id} failed: {e}"),
        }
    }
}
