CREATE TABLE IF NOT EXISTS m_repo
(
    id             TEXT PRIMARY KEY NOT NULL,
    name           TEXT NOT NULL,
    description    TEXT,
    create_time    TIMESTAMP NOT NULL,
    deleted        BOOLEAN             NOT NULL DEFAULT 0
);