CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

--------------------------------------------------
-- USERS
--------------------------------------------------

CREATE TABLE users
(
    id UUID PRIMARY KEY,

    username TEXT NOT NULL UNIQUE,

    password_hash TEXT NOT NULL,

    role TEXT NOT NULL
);

--------------------------------------------------
-- LOGS
--------------------------------------------------

CREATE TABLE logs
(
    id UUID PRIMARY KEY,

    source_name TEXT NOT NULL,

    vendor TEXT,

    hostname TEXT,

    username TEXT,

    ip_address TEXT,

    event_type TEXT NOT NULL,

    severity TEXT NOT NULL,

    message TEXT NOT NULL,

    raw_log JSONB NOT NULL,

    created_at TIMESTAMP NOT NULL,

    search_vector tsvector
);

CREATE INDEX logs_search_idx
ON logs
USING GIN(search_vector);

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

--------------------------------------------------
-- PERMISSIONS
--------------------------------------------------

GRANT ALL PRIVILEGES
ON ALL TABLES IN SCHEMA public
TO medtech;

GRANT ALL PRIVILEGES
ON ALL SEQUENCES IN SCHEMA public
TO medtech;

ALTER DEFAULT PRIVILEGES
IN SCHEMA public
GRANT ALL PRIVILEGES
ON TABLES
TO medtech;

ALTER DEFAULT PRIVILEGES
IN SCHEMA public
GRANT ALL PRIVILEGES
ON SEQUENCES
TO medtech;