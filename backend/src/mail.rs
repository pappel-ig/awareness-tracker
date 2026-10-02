use std::collections::HashMap;
use std::env;
use anyhow::{bail, Context, Result};
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use crate::Config;

const INVITE_TEMPLATE: &str = include_str!("../templates/invite.html");

#[derive(Clone)]
pub struct EmailTemplateService {
    mailer: SmtpTransport,
    from_name: String,
    from_email: String,
}

impl EmailTemplateService {

    pub fn send_invite(&self, config: &Config, to_mail: &str, token: &str) -> Result<()> {
        let mut vals = HashMap::new();
        vals.insert("survey", format!("https://{}/survey?token={}", config.frontend_addr, token));
        vals.insert("bind", config.addr.clone());
        vals.insert("token", token.to_string());

        self.send_template(
            "templates/invite.html",
            "Deine Einladung zur Security Awareness Umfrage",
            to_mail,
            vals,
        )
    }

    pub fn send_template(&self, template: &str, subject: &str, to_mail: &str, vals: HashMap<&str, String>) -> Result<()> {
        let contents = match template {
            "templates/invite.html" => INVITE_TEMPLATE,
            other => bail!("Unbekanntes Template: {other}"),
        };

        self.send_mail(contents.to_string(), subject, to_mail, vals)
            .with_context(|| format!("Template {template} konnte nicht versendet werden"))
    }


    pub fn new() -> Result<Self> {
        let smtp_server = env::var("SMTP_SERVER").unwrap_or_else(|_| "localhost".to_string());
        let smtp_port = env::var("SMTP_PORT").unwrap_or_else(|_| "1025".to_string()).parse::<u16>().context("SMTP_PORT invalid value")?;
        let smtp_username = env::var("SMTP_USERNAME").unwrap_or_else(|_| "mock_user".to_string());
        let smtp_password = env::var("SMTP_PASSWORD").unwrap_or_else(|_| "mock_pass".to_string());
        let from_name = env::var("SMTP_NAME").unwrap_or_else(|_| "Example".to_string());
        let from_email = env::var("SMTP_FROM").unwrap_or_else(|_| "test@example.org".to_string());
        let smtp_accept_invalid_certs = env::var("DEBUG").is_ok();
        
        let credentials = Credentials::new(smtp_username, smtp_password);
        let mut builder = SmtpTransport::starttls_relay(&smtp_server)
            .context("SmtpTransport konnte nicht gestartet werden")?
            .port(smtp_port)
            .credentials(credentials);

        if smtp_accept_invalid_certs {
            let tls_parameters = TlsParameters::builder(smtp_server)
                .dangerous_accept_invalid_certs(true)
                .build()
                .context("TlsParameters für SMTP konnten nicht erstellt werden")?;
            builder = builder.tls(Tls::Required(tls_parameters));
        }

        let mailer = builder.build();

        Ok(EmailTemplateService {
            mailer,
            from_name,
            from_email,
        })
    }

    fn send_mail(&self, template: String, subject: &str, to_mail: &str, vals: HashMap<&str, String>) -> Result<()> {
        let email = Message::builder()
            .from(format!("{} <{}>", self.from_name, self.from_email).parse().context("Ungültige Absender-Adresse")?)
            .to(to_mail.parse().context("Ungültige Empfänger-Adresse")?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(Self::render_template(template, &vals))
            .context("E-Mail konnte nicht erstellt werden")?;

        self.mailer.send(&email).context("E-Mail-Versand fehlgeschlagen")?;

        Ok(())
    }

    fn render_template(template: String, vars: &HashMap<&str, String>) -> String {
        let mut rendered = template;
        for (key, value) in vars {
            rendered = rendered.replace(&format!("{{{{{key}}}}}"), value);
        }
        rendered
    }
}
