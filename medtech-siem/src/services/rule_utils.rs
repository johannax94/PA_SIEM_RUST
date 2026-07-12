use chrono::NaiveDateTime;
use sqlx::PgPool;

pub async fn count_events(
    db: &PgPool,
    source_name: &str,
    event_type: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM logs
        WHERE source_name = $1
        AND event_type = $2
        AND created_at >= $3
        "#
    )
    .bind(source_name)
    .bind(event_type)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}

pub async fn count_events_by_ip(
    db: &PgPool,
    ip_address: &str,
    event_type: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM logs
        WHERE ip_address = $1
        AND event_type = $2
        AND created_at >= $3
        "#
    )
    .bind(ip_address)
    .bind(event_type)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}

pub async fn count_distinct_users_by_ip(
    db: &PgPool,
    ip_address: &str,
    event_type: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(DISTINCT username)
        FROM logs
        WHERE ip_address = $1
        AND event_type = $2
        AND created_at >= $3
        "#
    )
    .bind(ip_address)
    .bind(event_type)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}

pub async fn count_events_by_username(
    db: &PgPool,
    username: &str,
    event_type: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM logs
        WHERE username = $1
        AND event_type = $2
        AND created_at >= $3
        "#
    )
    .bind(username)
    .bind(event_type)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}

pub async fn count_distinct_countries_by_username(
    db: &PgPool,
    username: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(DISTINCT raw_log->>'country')
        FROM logs
        WHERE username = $1
        AND created_at >= $2
        AND raw_log->>'country' IS NOT NULL
        "#
    )
    .bind(username)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}

/// Nombre d'échecs d'ouverture de session RDP depuis une IP donnée, sur la
/// fenêtre indiquée.
///
/// Se base sur l'événement Windows 4625 normalisé (event_type `login_failed`)
/// filtré sur `logon_type = 10` (RemoteInteractive = RDP). C'est ce couple
/// « échec + type de session RDP + même IP » qui caractérise le brute-force
/// d'un service RDP exposé.
pub async fn count_rdp_failures_by_ip(
    db: &PgPool,
    ip_address: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM logs
        WHERE ip_address = $1
        AND event_type = 'login_failed'
        AND raw_log->>'logon_type' = '10'
        AND created_at >= $2
        "#
    )
    .bind(ip_address)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}

/// Somme des octets sortants (raw_log->>'bytes_out') d'une même source vers des
/// destinations EXTERNES (IP publiques) sur la fenêtre donnée.
///
/// Se base sur des logs de flux réseau normalisés (event_type `network_flow`),
/// en excluant :
///   - les valeurs non numériques (garde-fou du cast),
///   - les destinations privées / loopback / link-local (RFC 1918, 127/8,
///     169.254/16) : on ne veut que le trafic sortant vers Internet.
/// C'est ce VOLUME CUMULÉ vers l'externe qui caractérise l'exfiltration, pas un
/// transfert isolé.
pub async fn sum_external_bytes_out_by_source(
    db: &PgPool,
    source_name: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COALESCE(SUM((raw_log->>'bytes_out')::bigint), 0)::bigint
        FROM logs
        WHERE source_name = $1
        AND event_type = 'network_flow'
        AND raw_log->>'bytes_out' ~ '^[0-9]+$'
        AND (raw_log->>'dest_ip') !~ '^(10\.|192\.168\.|172\.(1[6-9]|2[0-9]|3[0-1])\.|127\.|169\.254\.)'
        AND created_at >= $2
        "#
    )
    .bind(source_name)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}

/// Nombre d'écritures fichiers portant un MARQUEUR de ransomware pour une même
/// source sur la fenêtre : extension de chiffrement connue (ou hex aléatoire),
/// OU nom de fichier de note de rançon.
///
/// C'est ce couple « volume + marqueur » qui distingue un chiffrement massif
/// d'une simple copie/sauvegarde légitime (qui, elle, n'a aucun marqueur).
pub async fn count_ransomware_writes_by_source(
    db: &PgPool,
    source_name: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM logs
        WHERE source_name = $1
        AND event_type IN ('file_modified', 'file_renamed')
        AND (
            raw_log->>'new_extension' ~* '^(locked|encrypted|crypt|crypted|crypto|enc|cry|wcry|wncry|lockbit|conti|ryuk|akira|phobos|djvu|locky|cerber|[a-f0-9]{8})$'
            OR raw_log->>'filename' ~* '(read.?me|how.?to.?decrypt|_readme|restore.?files|decrypt.?instruction|your.?files.?encrypted)'
        )
        AND created_at >= $2
        "#
    )
    .bind(source_name)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}

/// Nombre de PORTS de destination DISTINCTS sondés par une même IP source via
/// des connexions bloquées, sur la fenêtre donnée.
///
/// C'est le nombre de ports distincts (balayage) — et non le volume — qui
/// caractérise un scan : un client mal configuré reboucle sur UN port
/// (distinct = 1), un scanner en teste des dizaines.
pub async fn count_distinct_ports_by_ip(
    db: &PgPool,
    ip_address: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(DISTINCT raw_log->>'dest_port')
        FROM logs
        WHERE ip_address = $1
        AND event_type = 'blocked_connection'
        AND raw_log->>'dest_port' IS NOT NULL
        AND created_at >= $2
        "#
    )
    .bind(ip_address)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}