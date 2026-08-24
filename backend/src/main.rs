mod mail;

use std::collections::HashMap;
use std::env;

use crate::mail::EmailTemplateService;
use anyhow::{Context, Result};
use dotenvy::dotenv;
use lettre::Transport;

fn env_var(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("Umgebungsvariable {key} fehlt (siehe .env)"))
}

fn main() -> Result<()> {
    dotenv().ok();

    let smtp_server = env_var("SMTP_SERVER")?;
    let smtp_port: u16 = env_var("SMTP_PORT")?
        .parse()
        .context("SMTP_PORT ist keine gueltige Zahl")?;
    let smtp_username = env_var("SMTP_USERNAME")?;
    let smtp_password = env_var("SMTP_PASSWORD")?;

    let from_name = env_var("FROM_NAME")?;
    let from_email = env_var("FROM_EMAIL")?;
    let to_name = env_var("TO_NAME")?;

    let mut vars = HashMap::new();
    vars.insert("name", to_name.clone());
    vars.insert("from_name", from_name.clone());

    let mail_service = EmailTemplateService::new(
        smtp_server,
        smtp_port,
        smtp_username,
        smtp_password,
        from_name,
        from_email
    );

    mail_service.send_template("templates/email.html",
                               "Test Email",
                               "Test Benutzer",
                               "example@example.org", vars)
        .expect("Failed to send template");


    Ok(())
}
