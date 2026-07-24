use axum::{http::StatusCode, Json};
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::{authentication::Credentials, extension::ClientId},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::Deserialize;
use serde_json::{json, Value};

const MAX_MESSAGE_LEN: usize = 5000;

#[derive(Deserialize)]
pub struct ContactRequest {
    pub email: String,
    pub message: String,
}

pub async fn send_contact(
    Json(payload): Json<ContactRequest>,
) -> (StatusCode, Json<Value>) {

    let message = payload.message.trim();

    if message.is_empty() || message.len() > MAX_MESSAGE_LEN {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "message vide ou trop long" })),
        );
    }

    // Validation d'entrée (défense en profondeur) : le commentaire ne doit
    // contenir que des lettres (accentuées incluses), chiffres, espaces et la
    // ponctuation simple . , ! ? — aucun caractère spécial (', ", ;, <, >...).
    let comment_ok = message
        .chars()
        .all(|c| c.is_alphanumeric() || c.is_whitespace() || ".,!?".contains(c));

    if !comment_ok {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "le commentaire contient des caractères spéciaux non autorisés" })),
        );
    }

    // L'email doit impérativement contenir un @ (message d'erreur explicite ;
    // le parse::<Mailbox>() ci-dessous valide ensuite le format complet).
    if !payload.email.contains('@') {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "l'adresse email doit contenir un @" })),
        );
    }

    // parse::<Mailbox>() valide le format de l'adresse au passage
    let visitor: Mailbox = match payload.email.trim().parse() {
        Ok(mailbox) => mailbox,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "adresse email invalide" })),
            );
        }
    };

    let smtp_user = match std::env::var("SMTP_USER") {
        Ok(user) => user,
        Err(_) => {
            tracing::error!("SMTP_USER manquant dans l'environnement");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "service mail non configuré" })),
            );
        }
    };

    let smtp_password = match std::env::var("SMTP_APP_PASSWORD") {
        Ok(password) => password,
        Err(_) => {
            tracing::error!("SMTP_APP_PASSWORD manquant dans l'environnement");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "service mail non configuré" })),
            );
        }
    };

    let sender: Mailbox = match smtp_user.parse() {
        Ok(mailbox) => mailbox,
        Err(_) => {
            tracing::error!("SMTP_USER n'est pas une adresse valide");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "service mail non configuré" })),
            );
        }
    };

    let notification_body = format!(
        "Nouveau message depuis la page contact MedTech SIEM\n\n\
         De : {}\n\n\
         Message :\n{}\n",
        payload.email.trim(),
        message
    );

    let notification = match Message::builder()
        // Gmail impose que l'expéditeur soit le compte authentifié ;
        // reply_to permet de répondre directement au visiteur
        .from(sender.clone())
        .reply_to(visitor.clone())
        .to(sender.clone())
        .subject("MedTech SIEM — Nouveau message de contact")
        .header(ContentType::TEXT_PLAIN)
        .body(notification_body)
    {
        Ok(email) => email,
        Err(e) => {
            tracing::error!("construction du mail impossible: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "construction du mail impossible" })),
            );
        }
    };

    let confirmation_body = format!(
        "Bonjour,\n\n\
         Votre message a bien été transmis à l'équipe MedTech SIEM.\n\
         Nous vous répondrons au plus vite à cette adresse.\n\n\
         Rappel de votre message :\n{}\n\n\
         — L'équipe MedTech SIEM",
        message
    );

    let confirmation = Message::builder()
        .from(sender)
        .to(visitor)
        .subject("MedTech SIEM — Votre message a bien été envoyé")
        .header(ContentType::TEXT_PLAIN)
        .body(confirmation_body)
        .ok();

    let mailer = match AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com") {
        Ok(builder) => builder
            .credentials(Credentials::new(smtp_user, smtp_password))
            // nom fixe pour le EHLO : le hostname de la machine peut être
            // non-ASCII, ce que les serveurs SMTP refusent
            .hello_name(ClientId::Domain("medtech-siem".to_string()))
            .build(),
        Err(e) => {
            tracing::error!("connexion SMTP impossible: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "connexion SMTP impossible" })),
            );
        }
    };

    // la notification à l'équipe doit partir, sinon on remonte l'erreur
    if let Err(e) = mailer.send(notification).await {
        tracing::error!("envoi du mail échoué: {e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "envoi du mail échoué" })),
        );
    }

    // la confirmation au visiteur est best-effort : sa demande est déjà
    // arrivée, on ne fait pas échouer la requête pour autant
    if let Some(confirmation) = confirmation {
        if let Err(e) = mailer.send(confirmation).await {
            tracing::warn!("mail de confirmation au visiteur échoué: {e}");
        }
    }

    (StatusCode::OK, Json(json!({ "status": "sent" })))
}
