use std::net::SocketAddr;

use chrono::{Datelike, Utc};
use lettre::message::header::ContentType;
use lettre::transport::smtp::{authentication::Credentials, response::Response};
use lettre::{SmtpTransport, Transport};
use tera::{Context, Tera};

use crate::models::User;
use crate::rocket_routes::get_client_info;

pub struct HtmlMailer {
    pub credentials: Credentials,
    pub smtp_host: String,
    pub template_engine: tera::Tera,
}

impl HtmlMailer {
    pub fn new() -> Result<HtmlMailer, Box<dyn std::error::Error>> {
        let smtp_host = std::env::var("SMTP_HOST").expect("Cannot load SMTP host from env");
        let smtp_username =
            std::env::var("SMTP_USERNAME").expect("Cannot load SMTP username from env");
        let smtp_password =
            std::env::var("SMTP_PASSWORD").expect("Cannot load SMTP password from env");

        let credentials =
            lettre::transport::smtp::authentication::Credentials::new(smtp_username, smtp_password);
        let tera = Tera::new("templates/**/*.html").unwrap_or_else(|e| {
            panic!("Parsing error(s): {}", e);
        });

        Ok(HtmlMailer {
            smtp_host,
            credentials,
            template_engine: tera,
        })
    }

    pub fn send(
        self,
        to: Vec<String>,
        subject: Option<String>,
        template_name: &str,
        context: &Context,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let html_body = self.template_engine.render(template_name, context)?;
        let subject = subject.unwrap_or_else(|| "(no subject)".to_string());

        let mut message_builder = lettre::Message::builder()
            .subject(subject)
            .from("Template App <softteco.os.dev@gmail.com>".parse().unwrap())
            .to(to[0].parse()?)
            .header(ContentType::TEXT_HTML);

        if to.len() > 1 {
            for copy in &to[1..] {
                message_builder = message_builder.cc(copy.parse()?);
            }
        }

        let message = message_builder.body(html_body)?;

        let mailer = SmtpTransport::relay(&self.smtp_host)?
            .credentials(self.credentials)
            .build();

        mailer.send(&message).map_err(|e| e.into())
    }
}

pub async fn send_reset_password_email(user: User, deep_link: String, client_addr: SocketAddr) {
    let client_info = get_client_info(client_addr).await.unwrap();

    let year = Utc::now().year();

    log::info!("Sending reset password email for {}", user.username);

    let mut context = Context::new();
    context.insert("username", &user.username);
    context.insert("deep_link", &deep_link);
    context.insert("client_info", &client_info);
    context.insert("year", &year);

    let mailer = HtmlMailer::new().unwrap();

    mailer
        .send(
            vec![user.email],
            Some(String::from("Reset password")),
            "email/reset_password.html",
            &context,
        )
        .unwrap();
}
