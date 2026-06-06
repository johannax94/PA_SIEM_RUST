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