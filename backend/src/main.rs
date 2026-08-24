use std::collections::HashMap;
use std::env;
use std::fs;

use anyhow::{Context, Result};
use dotenvy::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

fn render_template(template: &str, vars: &HashMap<&str, String>) -> String {
    let mut rendered = template.to_string();
    for (key, value) in vars {
        rendered = rendered.replace(&format!("{{{{{key}}}}}"), value);
    }
    rendered
}

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
    let to_email = env_var("TO_EMAIL")?;
    let subject = env_var("SUBJECT")?;

    let template = fs::read_to_string("templates/email.html")
        .context("Template templates/email.html konnte nicht gelesen werden")?;

    let mut vars = HashMap::new();
    vars.insert("name", to_name.clone());
    vars.insert("from_name", from_name.clone());
    let body = render_template(&template, &vars);

    let email = Message::builder()
        .from(format!("{from_name} <{from_email}>").parse()?)
        .to(format!("{to_name} <{to_email}>").parse()?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body)?;

    let credentials = Credentials::new(smtp_username, smtp_password);

    let mailer = SmtpTransport::starttls_relay(&smtp_server)?
        .port(smtp_port)
        .credentials(credentials)
        .build();

    mailer.send(&email).context("E-Mail-Versand fehlgeschlagen")?;

    println!("E-Mail erfolgreich an {to_email} gesendet.");

    Ok(())
}
