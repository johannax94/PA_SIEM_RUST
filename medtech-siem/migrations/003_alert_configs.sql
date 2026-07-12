--------------------------------------------------
-- NOTIFICATIONS EMAIL CONFIGURABLES
--
-- L'analyste crée des règles de notification : "si la règle X se déclenche
-- N fois sur une même entité en M minutes, envoyer un mail". Le mail part
-- vers l'adresse du SIEM (medtechsiem@gmail.com).
--------------------------------------------------

CREATE TABLE alert_configs
(
    id UUID PRIMARY KEY,

    -- nom de la règle de détection surveillée (ex. 'cmd_suspect')
    rule_name TEXT NOT NULL,

    -- nombre d'apparitions (sur une même entité) déclenchant le mail
    threshold INT NOT NULL,

    -- fenêtre d'observation (minutes)
    window_minutes INT NOT NULL,

    -- description libre du problème, incluse dans le mail
    comment TEXT NOT NULL,

    enabled BOOLEAN NOT NULL DEFAULT TRUE,

    created_at TIMESTAMP NOT NULL
);

-- Trace des mails déjà envoyés : anti-spam (on ne renotifie pas pour une
-- même (config, entité) avant la fin de la fenêtre).
CREATE TABLE alert_notifications
(
    id UUID PRIMARY KEY,

    config_id UUID NOT NULL,

    entity TEXT NOT NULL,

    sent_at TIMESTAMP NOT NULL
);

CREATE INDEX alert_notifications_lookup_idx
ON alert_notifications (config_id, entity, sent_at);
