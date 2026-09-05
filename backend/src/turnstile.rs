use std::env;

use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::Deserialize;

const SITEVERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

pub struct TurnstileClient {
    client: Client,
    secret_key: String,
}

#[derive(Deserialize)]
struct SiteverifyResponse {
    success: bool,
}

impl TurnstileClient {
    pub fn new() -> Result<Self> {
        let secret_key = env::var("TURNSTILE_SECRET_KEY")
            .map_err(|_| anyhow!("TURNSTILE_SECRET_KEY environment variable is not set"))?;
        Ok(Self { client: Client::new(), secret_key })
    }

    pub async fn verify(&self, token: &str, remote_ip: Option<&str>) -> Result<bool> {
        if token.trim().is_empty() {
            return Ok(false);
        }

        let mut params = vec![
            ("secret", self.secret_key.as_str()),
            ("response", token),
        ];
        if let Some(ip) = remote_ip {
            params.push(("remoteip", ip));
        }

        let response = self
            .client
            .post(SITEVERIFY_URL)
            .form(&params)
            .send()
            .await?;
        let body: SiteverifyResponse = response.json().await?;
        Ok(body.success)
    }
}
