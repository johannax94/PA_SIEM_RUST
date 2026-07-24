// Test manuel de la config SMTP du formulaire de contact :
// cargo run --example smtp_test
use lettre::{
    message::header::ContentType,
    transport::smtp::{authentication::Credentials, extension::ClientId},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let smtp_user = std::env::var("SMTP_USER").expect("SMTP_USER manquant");
    let smtp_password =
        std::env::var("SMTP_APP_PASSWORD").expect("SMTP_APP_PASSWORD manquant");

    let email = Message::builder()
        .from(smtp_user.parse().unwrap())
        .to(smtp_user.parse().unwrap())
        .subject("Test SMTP — MedTech SIEM contact")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(
            "Test de la configuration SMTP du formulaire de contact : \
             si vous lisez ceci, l'authentification Gmail fonctionne.",
        ))
        .unwrap();

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .unwrap()
        .credentials(Credentials::new(smtp_user, smtp_password))
        .hello_name(ClientId::Domain("medtech-siem".to_string()))
        .build();

    match mailer.send(email).await {
        Ok(_) => println!("SMTP AUTH OK — mail de test envoyé"),
        Err(e) => {
            eprintln!("ÉCHEC : {e}");
            std::process::exit(1);
        }
    }
}
