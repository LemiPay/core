use server::domain::user::Email;
use server::infrastructure::email::azure_email_sender::AzureEmailSender;
use server::infrastructure::email::email_sender::EmailService;

/// Este bin es para probar el envío de mails. Se puede ejecutar con `cargo run --bin send --features production`.
pub async fn test_mail() {
    const EMAIL: &str = "mateo.julian.d@gmail.com";

    let email_service = AzureEmailSender::new();

    let email = Email(EMAIL.to_string());

    email_service
        .send_welcome_email(&email, "Joe")
        .await
        .expect("Pincho el mail");

    email_service
        .send_login_alert(&email, "Joe")
        .await
        .expect("Pincho el mail");

    email_service.example(&email).await.expect("Pincho el mail");

    println!("Done! Esperando a que los hilos de fondo terminen...");

    // Damos 3 segundos para que los tokio::spawn hagan las peticiones HTTP
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(test_mail());
}
