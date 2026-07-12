//! ============================================================================
//! Notifications email configurables
//! ============================================================================
//!
//! L'analyste définit des règles de notification (table alert_configs) :
//! « si la règle X se déclenche N fois sur une même entité en M minutes,
//! envoyer un mail ». Ce module est appelé à chaque nouvelle alerte : il
//! vérifie les configs concernées, compte les occurrences par entité et
//! envoie le mail (anti-spam : pas de renvoi avant la fin de la fenêtre).

use chrono::{Duration, Utc};
use lettre::message::Mailbox;
use sqlx::PgPool;
use uuid::Uuid;

use crate::services::mailer;

/// Catalogue des règles de détection notifiables (noms émis par create_alert).
/// Sert à peupler le menu déroulant du formulaire côté frontend.
pub const RULE_CATALOG: [&str; 17] = [
    "multiple_failed_logins",
    "password_spray",
    "bruteforce_success",
    "rdp_bruteforce_external",
    "powershell_suspect",
    "cmd_suspect",
    "network_scan",
    "ransomware",
    "data_exfiltration",
    "privilege_escalation",
    "rdp_foreign_country",
    "impossible_travel",
    "new_account_admin_group",
    "audit_log_cleared",
    "shadow_copy_deletion",
    "defense_disabled",
    "office_spawns_shell",
];

/// Appelé à chaque nouvelle alerte. Pour chaque config active surveillant
/// cette règle, envoie un mail si le seuil est atteint sur l'entité.
pub async fn check_and_notify(db: &PgPool, rule: &str, entity: &str) {

    // Configs actives surveillant cette règle.
    let configs: Vec<(Uuid, i32, i32, String)> = sqlx::query_as(
        r#"
        SELECT id, threshold, window_minutes, comment
        FROM alert_configs
        WHERE rule_name = $1 AND enabled = TRUE
        "#,
    )
    .bind(rule)
    .fetch_all(db)
    .await
    .unwrap_or_default();

    for (config_id, threshold, window_minutes, comment) in configs {
        let since = Utc::now().naive_utc() - Duration::minutes(window_minutes as i64);

        // Occurrences de cette règle sur CETTE entité dans la fenêtre.
        let count: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM alerts
            WHERE rule_name = $1 AND source_name = $2 AND created_at >= $3
            "#,
        )
        .bind(rule)
        .bind(entity)
        .bind(since)
        .fetch_one(db)
        .await
        .unwrap_or((0,));

        if count.0 < threshold as i64 {
            continue;
        }

        // Anti-spam : déjà notifié pour ce couple (config, entité) sur la fenêtre ?
        let already: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM alert_notifications
            WHERE config_id = $1 AND entity = $2 AND sent_at >= $3
            "#,
        )
        .bind(config_id)
        .bind(entity)
        .bind(since)
        .fetch_one(db)
        .await
        .unwrap_or((0,));

        if already.0 > 0 {
            continue;
        }

        send_notification(db, config_id, rule, entity, count.0, threshold, window_minutes, &comment)
            .await;
    }
}

#[allow(clippy::too_many_arguments)]
async fn send_notification(
    db: &PgPool,
    config_id: Uuid,
    rule: &str,
    entity: &str,
    count: i64,
    threshold: i32,
    window_minutes: i32,
    comment: &str,
) {
    let recipient: Mailbox = match mailer::siem_mailbox() {
        Ok(mb) => mb,
        Err(e) => {
            tracing::error!("notification impossible : {e}");
            return;
        }
    };

    let subject = format!("[MedTech SIEM] Alerte : {rule} x{count} sur {entity}");
    let body = format!(
        "Une règle de notification MedTech SIEM s'est déclenchée.\n\n\
         Règle surveillée : {rule}\n\
         Entité concernée : {entity}\n\
         Occurrences : {count} (seuil : {threshold} en {window_minutes} min)\n\n\
         Commentaire de l'analyste :\n{comment}\n\n\
         Connectez-vous à la console pour investiguer.\n\
         — MedTech SIEM",
    );

    if let Err(e) = mailer::send_text(&recipient, &subject, &body).await {
        tracing::error!("envoi de la notification échoué : {e}");
        return;
    }

    // Trace anti-spam : on mémorise l'envoi.
    let _ = sqlx::query(
        r#"
        INSERT INTO alert_notifications (id, config_id, entity, sent_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(config_id)
    .bind(entity)
    .bind(Utc::now().naive_utc())
    .execute(db)
    .await;

    tracing::warn!("notification envoyée : {rule} x{count} sur {entity}");
}
