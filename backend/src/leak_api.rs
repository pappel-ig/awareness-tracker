use std::env;
use std::sync::Arc;
use std::time::Duration;
use anyhow::anyhow;
use log::{error, warn};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::json;
use tokio_postgres::Client as DbClient;
use uuid::Uuid;

const POLL_INTERVAL: Duration = Duration::from_secs(5);
const MAX_BACKOFF: Duration = Duration::from_secs(30 * 60);
const DEFAULT_RATE_LIMIT_WAIT: Duration = Duration::from_secs(10);

const HIBP_API_URL: &str = "https://haveibeenpwned.com/api/v3/breachedaccount";

#[derive(Deserialize)]
struct Breach {
    #[serde(rename = "Name")]
    name: String,
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

async fn check_email(client: &Client, email: &str) -> Result<Vec<String>, CheckError> {
    let mut url = reqwest::Url::parse(HIBP_API_URL)?;
    url.path_segments_mut()
        .map_err(|_| anyhow!("HIBP_API_URL is not a valid base URL"))?
        .push(email);
    url.query_pairs_mut().append_pair("truncateResponse", "true");

    let response = client.get(url).send().await?;

    match response.status() {
        StatusCode::OK => {
            let breaches: Vec<Breach> = response.json().await?;
            Ok(breaches.into_iter().map(|b| b.name).collect())
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

pub async fn run_worker(db: Arc<DbClient>, client: Arc<Client>) {
    let mut backoff = POLL_INTERVAL;

    loop {
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
