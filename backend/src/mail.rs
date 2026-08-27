use std::collections::HashMap;
use std::error::Error;
use std::fs;
use anyhow::{Context, Result};
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;

#[derive(Clone)]
pub struct EmailTemplateService {
    mailer: SmtpTransport,
    from_name: String,
    from_email: String,
}

impl EmailTemplateService {

    pub fn send_template(&self, template: &str, subject: &str, name: &str, to_mail: &str, vals: HashMap<&str, String>) -> Result<(), Box<dyn Error>> {
        let template = fs::read_to_string("templates/invite.html")
            .with_context(|| format!("Template {} konnte nicht gelesen werden", template))?;

        Self::send_mail(self, template, subject, name, to_mail, vals).expect("Failed to send template");
        Ok(())
    }


    pub fn new(smtp_server: String,
               smtp_port: u16,
               smtp_username: String,
               smtp_password: String,
               from_name: String,
               from_email: String) -> Self {
        let credentials = Credentials::new(smtp_username, smtp_password);
        let mailer = SmtpTransport::starttls_relay(&smtp_server)
            .expect("Could not start SmtpTransport for Smtp")
            .port(smtp_port)
            .credentials(credentials)
            .build();

        EmailTemplateService {
            mailer,
            from_name,
            from_email,
        }
    }

    fn send_mail(&self, template: String, subject: &str, name: &str, to_mail: &str, vals: HashMap<&str, String>) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .from(format!("{} <{}>", self.from_name, self.from_email).parse()?)
            .to(format!("{} <{}>", name, to_mail).parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(Self::render_template(template, &vals))?;

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