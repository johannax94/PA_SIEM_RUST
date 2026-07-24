CREATE TABLE IF NOT EXISTS logs (
    id UUID PRIMARY KEY,
    source_name TEXT NOT NULL,
    event_type TEXT NOT NULL,
    severity TEXT NOT NULL,
    message TEXT NOT NULL,
    raw_log JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS alerts (
    id UUID PRIMARY KEY,
    rule_name TEXT NOT NULL,
    severity TEXT NOT NULL,
    description TEXT NOT NULL,
    source_name TEXT,
    timestamp TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL
);

-- RBA (Risk-Based Alerting) : observations de risque et incidents
CREATE TABLE IF NOT EXISTS risk_events (
    id UUID PRIMARY KEY,
    entity TEXT NOT NULL,
    rule_name TEXT NOT NULL,
    severity TEXT NOT NULL,
    score INT NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS risk_events_entity_time_idx
ON risk_events (entity, created_at);

CREATE TABLE IF NOT EXISTS risk_incidents (
    id UUID PRIMARY KEY,
    entity TEXT NOT NULL,
    risk_score INT NOT NULL,
    severity TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    rules_involved TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS risk_incidents_entity_status_idx
ON risk_incidents (entity, status);

-- Notifications email configurables
CREATE TABLE IF NOT EXISTS alert_configs (
    id UUID PRIMARY KEY,
    rule_name TEXT NOT NULL,
    threshold INT NOT NULL,
    window_minutes INT NOT NULL,
    comment TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS alert_notifications (
    id UUID PRIMARY KEY,
    config_id UUID NOT NULL,
    entity TEXT NOT NULL,
    sent_at TIMESTAMP NOT NULL
);

CREATE INDEX IF NOT EXISTS alert_notifications_lookup_idx
ON alert_notifications (config_id, entity, sent_at);