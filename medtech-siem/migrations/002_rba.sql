--------------------------------------------------
-- RBA (Risk-Based Alerting)
--
-- Chaque détection dépose une observation de risque
-- scorée sur une entité (machine source). Un incident
-- n'est créé que lorsque le score cumulé de l'entité
-- sur la fenêtre glissante franchit un seuil.
--------------------------------------------------

CREATE TABLE risk_events
(
    id UUID PRIMARY KEY,

    entity TEXT NOT NULL,

    rule_name TEXT NOT NULL,

    severity TEXT NOT NULL,

    score INT NOT NULL,

    message TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL
);

CREATE INDEX risk_events_entity_time_idx
ON risk_events (entity, created_at);

CREATE TABLE risk_incidents
(
    id UUID PRIMARY KEY,

    entity TEXT NOT NULL,

    risk_score INT NOT NULL,

    -- low | medium | high
    severity TEXT NOT NULL,

    -- open | closed
    status TEXT NOT NULL DEFAULT 'open',

    -- règles distinctes ayant contribué au score
    rules_involved TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL,

    updated_at TIMESTAMP NOT NULL
);

CREATE INDEX risk_incidents_entity_status_idx
ON risk_incidents (entity, status);
