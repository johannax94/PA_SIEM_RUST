//! Transport SMTP partagé (Gmail) — utilisé par la page contact et par les
//! notifications d'alerte configurables. Les identifiants viennent de
//! l'environnement : SMTP_USER et SMTP_APP_PASSWORD.

use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::{authentication::Credentials, extension::ClientId},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

/// Adresse du compte SIEM : expéditeur et destinataire des notifications.
pub fn siem_mailbox() -> Result<Mailbox, String> {
    std::env::var("SMTP_USER")
        .map_err(|_| "SMTP_USER manquant".to_string())?
        .parse()
        .map_err(|_| "SMTP_USER n'est pas une adresse valide".to_string())
}

/// Construit le transport SMTP Gmail authentifié.
fn transport() -> Result<AsyncSmtpTransport<Tokio1Executor>, String> {
    let user = std::env::var("SMTP_USER")
        .map_err(|_| "SMTP_USER manquant".to_string())?;
    let password = std::env::var("SMTP_APP_PASSWORD")
        .map_err(|_| "SMTP_APP_PASSWORD manquant".to_string())?;

    let builder = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")
        .map_err(|e| format!("connexion SMTP impossible: {e}"))?;

    Ok(builder
        .credentials(Credentials::new(user, password))
        // nom fixe pour le EHLO : le hostname de la machine peut être
        // non-ASCII, ce que les serveurs SMTP refusent
        .hello_name(ClientId::Domain("medtech-siem".to_string()))
        .build())
}

/// Envoie un mail texte depuis le compte SIEM vers `to`.
pub async fn send_text(
    to: &Mailbox,
    subject: &str,
    body: &str,
) -> Result<(), String> {

    let email = Message::builder()
        .from(siem_mailbox()?)
        .to(to.clone())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())
        .map_err(|e| format!("construction du mail impossible: {e}"))?;

    transport()?
        .send(email)
        .await
        .map_err(|e| format!("envoi du mail échoué: {e}"))?;

    Ok(())
}
