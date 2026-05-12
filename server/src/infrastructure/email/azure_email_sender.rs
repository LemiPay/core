use askama::Template;
use async_trait::async_trait;

#[allow(unused_imports)]
use std::env;

use crate::domain::user::Email;
use crate::infrastructure::email::email_sender::{EmailService, EmailServiceError};

use crate::infrastructure::email::template::{LoginAlertTemplate, RegisterTemplate};
use reqwest::Client;
use serde::Serialize;

#[allow(dead_code)]
pub struct AzureEmailSender {
    client: Client,
    secret: String,
    base_url: String,
}

// DTOs
#[derive(Serialize)]
pub struct AzureWelcomeRequest {
    pub to: String,
    pub subject: String,
    pub text: String,
}

impl AzureEmailSender {
    pub fn new() -> Self {
        #[cfg(not(test))]
        let (base_url, secret) = {
            dotenvy::dotenv().ok();
            dotenvy::from_filename("../.env").ok();

            (
                env::var("MAIL_API_URL").expect("MAIL_API_URL must be set"),
                env::var("MAIL_SERVER_SECRET").expect("MAIL_SERVER_SECRET must be set"),
            )
        };

        #[cfg(test)]
        let (base_url, secret) = ("http://localhost:8000".to_string(), "secret".to_string());

        Self {
            client: Client::new(),
            secret,
            base_url,
        }
    }

    pub fn send<T: Serialize>(&self, _to: &Email, _body: &T) {
        #[cfg(feature = "production")]
        {
            println!("Sending!");

            // reqwest::Client es un Arc por dentro, es muy barato de clonar.
            let client = self.client.clone();
            let secret = self.secret.clone();
            let endpoint = format!("{}/send-mail", self.base_url);
            let to_string = _to.to_string();

            // Serializamos el body a un valor que podamos adueñarnos ('static)
            // antes de mandarlo al hilo de fondo.
            let json_body = match serde_json::to_value(_body) {
                Ok(val) => val,
                Err(_) => {
                    println!("Error serializando el email para {}", to_string);
                    return;
                }
            };

            // Usamos async move para transferir la propiedad al hilo
            tokio::spawn(async move {
                println!("Enviando email a {} - REAL", to_string);

                // 5. Agregamos el .await para que el future realmente se ejecute
                let result = client
                    .post(&endpoint)
                    .header("Content-Type", "application/json")
                    .header("x-internal-secret", secret)
                    .json(&json_body)
                    .send()
                    .await;

                if let Err(e) = result {
                    println!("Error enviando el email real: {}", e);
                }
            });
        }

        // Bloque para todos los entornos que NO son production
        #[cfg(not(feature = "production"))]
        {
            // Esto le dice al compilador: "Sé que existen, no me tires warning de unused"
            let _ = _body;

            // Si no estamos en test (es decir, estamos en default/dev/demo)
            #[cfg(not(test))]
            {
                println!("Enviando email a {} - Mockeado.", _to);
            }
        }
    }
}

#[async_trait]
impl EmailService for AzureEmailSender {
    async fn example(&self, to: &Email) -> Result<(), EmailServiceError> {
        let body = AzureWelcomeRequest {
            to: to.to_string(),
            subject: "LemiPay!".to_string(),
            text: "Welcome to LemiPay!".to_string(),
        };

        self.send(&to, &[body]);

        Ok(())
    }

    async fn send_welcome_email(&self, to: &Email, name: &str) -> Result<(), EmailServiceError> {
        let template = RegisterTemplate {
            user_name: name,
            action_url: "",
        };

        // Renderizamos a String (Falla si el HTML tiene errores de sintaxis en las variables)
        let html_content = template.render().map_err(|_| EmailServiceError::Internal)?;

        let body = AzureWelcomeRequest {
            to: to.to_string(),
            subject: "¡Bienvenido a LemiPay!".to_string(),
            text: html_content,
        };

        self.send(&to, &[body]);

        Ok(())
    }

    async fn send_login_alert(&self, to: &Email, name: &str) -> Result<(), EmailServiceError> {
        let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let template = LoginAlertTemplate {
            user_name: name,
            time: &time.as_str(),
        };

        let html_content = template.render().map_err(|_| EmailServiceError::Internal)?;

        let body = AzureWelcomeRequest {
            to: to.to_string(),
            subject: "LemiPay: Nuevo inicio de sesión detectado".to_string(),
            text: html_content,
        };

        self.send(&to, &[body]);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EMAIL: &str = "joe@doe.com";

    #[tokio::test]
    async fn test_example() {
        let email_service = AzureEmailSender::new();
        let email = Email(TEST_EMAIL.parse().unwrap());

        let result = email_service.example(&email).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_email() {
        let email_service = AzureEmailSender::new();
        let email = Email(TEST_EMAIL.parse().unwrap());

        let result = email_service.send_welcome_email(&email, "Joe").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_alert_email() {
        let email_service = AzureEmailSender::new();
        let email = Email(TEST_EMAIL.parse().unwrap());

        let result = email_service.send_login_alert(&email, "Joe").await;

        assert!(result.is_ok());
    }
}
