-- Mornfold Phase 1 schema.
-- All timestamps are unix epoch milliseconds stored as INTEGER.
-- STRICT tables: SQLite enforces the declared column types.

-- Key/value settings (pause state, sound, show-on-next-start, ...).
-- `value` holds JSON produced by serde_json.
CREATE TABLE settings (
    key        TEXT PRIMARY KEY,
    value      TEXT NOT NULL,
    updated_at INTEGER NOT NULL
) STRICT;

-- Persistent scheduler jobs (plan 01-05). `next_run_at` is wall-clock time so
-- missed runs after sleep/shutdown can be caught up.
CREATE TABLE jobs (
    id                   TEXT PRIMARY KEY,
    kind                 TEXT NOT NULL,
    interval_sec         INTEGER NOT NULL,
    timeout_sec          INTEGER NOT NULL DEFAULT 60,
    next_run_at          INTEGER NOT NULL,
    last_run_at          INTEGER,
    last_status          TEXT,
    last_error           TEXT,
    consecutive_failures INTEGER NOT NULL DEFAULT 0,
    enabled              INTEGER NOT NULL DEFAULT 1,
    pausable             INTEGER NOT NULL DEFAULT 1
) STRICT;

-- One row per job execution (including coalesced catch-up runs).
CREATE TABLE job_runs (
    id            TEXT PRIMARY KEY,
    job_id        TEXT NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    scheduled_for INTEGER NOT NULL,
    started_at    INTEGER NOT NULL,
    finished_at   INTEGER,
    status        TEXT NOT NULL,
    catch_up      INTEGER NOT NULL DEFAULT 0,
    error         TEXT
) STRICT;

CREATE INDEX job_runs_started ON job_runs(started_at DESC);

-- Vault key check values (plan 01-06): detect a lost or mismatched master key
-- without ever touching real secrets.
CREATE TABLE vault_meta (
    key_version INTEGER PRIMARY KEY,
    kcv         BLOB NOT NULL,
    kcv_nonce   BLOB NOT NULL,
    created_at  INTEGER NOT NULL
) STRICT;

-- Encrypted secrets (AES-256-GCM, envelope encryption). Plaintext never
-- touches this table; `masked_hint` is computed at write time.
CREATE TABLE secrets (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    ciphertext  BLOB NOT NULL,
    nonce       BLOB NOT NULL,
    key_version INTEGER NOT NULL,
    masked_hint TEXT NOT NULL,
    created_at  INTEGER NOT NULL,
    updated_at  INTEGER NOT NULL
) STRICT;

-- In-app notification history (plan 01-07). `route` is the in-app
-- destination opened when the toast is clicked.
CREATE TABLE notifications (
    id          TEXT PRIMARY KEY,
    source      TEXT NOT NULL,
    title       TEXT NOT NULL,
    body        TEXT NOT NULL,
    route       TEXT NOT NULL,
    created_at  INTEGER NOT NULL,
    read_at     INTEGER,
    consumed_at INTEGER
) STRICT;

CREATE INDEX notifications_created ON notifications(created_at DESC);
