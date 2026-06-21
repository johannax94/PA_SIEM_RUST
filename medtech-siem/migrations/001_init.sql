CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

--------------------------------------------------
-- USERS
--------------------------------------------------

CREATE TABLE users
(
    id UUID PRIMARY KEY,

    username TEXT NOT NULL UNIQUE,

    password_hash TEXT NOT NULL,

    role TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

--------------------------------------------------
-- LOGS
--------------------------------------------------

CREATE TABLE logs
(
    id UUID PRIMARY KEY,

    source_name TEXT NOT NULL,

    event_type TEXT NOT NULL,

    severity TEXT NOT NULL,

    message TEXT NOT NULL,

    raw_log JSONB NOT NULL,

    created_at TIMESTAMP NOT NULL
);

--------------------------------------------------
-- ALERTS
--------------------------------------------------

CREATE TABLE alerts
(
    id UUID PRIMARY KEY,

    rule_name TEXT NOT NULL,

    severity TEXT NOT NULL,

    source_name TEXT NOT NULL,

    message TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL
);