use std::collections::HashMap;
use std::error::Error;
use anyhow::{Context, Result};
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};

const INVITE_TEMPLATE: &str = include_str!("../templates/invite.html");

#[derive(Clone)]
pub struct EmailTemplateService {
    mailer: SmtpTransport,
    from_name: String,
    from_email: String,
}

impl EmailTemplateService {

    pub fn send_template(&self, template: &str, subject: &str, to_mail: &str, vals: HashMap<&str, String>) -> Result<(), Box<dyn Error>> {
        let contents = match template {
            "templates/invite.html" => INVITE_TEMPLATE,
            other => return Err(format!("Unbekanntes Template: {other}").into()),
        };

        Self::send_mail(self, contents.to_string(), subject, to_mail, vals).expect("Failed to send template");
        Ok(())
    }


    pub fn new(smtp_server: String,
               smtp_port: u16,
               smtp_username: String,
               smtp_password: String,
               from_name: String,
               from_email: String,
               smtp_accept_invalid_certs: bool) -> Self {
        let credentials = Credentials::new(smtp_username, smtp_password);
        let mut builder = SmtpTransport::starttls_relay(&smtp_server)
            .expect("Could not start SmtpTransport for Smtp")
            .port(smtp_port)
            .credentials(credentials);

        if smtp_accept_invalid_certs {
            let tls_parameters = TlsParameters::builder(smtp_server)
                .dangerous_accept_invalid_certs(true)
                .build()
                .expect("Could not build TlsParameters for Smtp");
            builder = builder.tls(Tls::Required(tls_parameters));
        }

        let mailer = builder.build();

        EmailTemplateService {
            mailer,
            from_name,
            from_email,
        }
    }

    fn send_mail(&self, template: String, subject: &str, to_mail: &str, vals: HashMap<&str, String>) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .from(format!("{} <{}>", self.from_name, self.from_email).parse()?)
            .to(to_mail.parse()?)
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