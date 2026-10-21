//! SQLite connection pool and embedded migrations.
//!
//! SQL policy (Phase 1): runtime-checked static SQL only — `sqlx::query`,
//! `sqlx::query_as` + `#[derive(sqlx::FromRow)]`, `sqlx::query_scalar`, always
//! a `&'static str` with `.bind(..)`. Never build SQL with `format!` and never
//! wrap dynamic strings to bypass sqlx's `SqlSafeStr` check. Every statement
//! is exercised by tests against a migrated database.

use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use sqlx::SqlitePool;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};

use crate::error::CoreError;

/// Embedded migrations from `crates/mornfold-core/migrations`.
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// Opens (creating if missing) the on-disk database at `path`, configures WAL
/// mode, foreign keys, a 5 s busy timeout and `synchronous = NORMAL`, and runs
/// all pending migrations.
pub async fn open(path: &Path) -> Result<SqlitePool, CoreError> {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5))
        .synchronous(SqliteSynchronous::Normal);

    let pool = SqlitePoolOptions::new()
        .max_connections(4)
        .connect_with(options)
        .await?;

    MIGRATOR.run(&pool).await?;
    Ok(pool)
}

/// Opens a fresh, migrated in-memory database. Test helper, always public.
///
/// Uses a single connection that is never recycled, because every new
/// `sqlite::memory:` connection would be a separate, empty database.
pub async fn open_memory() -> Result<SqlitePool, CoreError> {
    let options = SqliteConnectOptions::from_str("sqlite::memory:")?.foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .min_connections(1)
        .idle_timeout(None)
        .max_lifetime(None)
        .connect_with(options)
        .await?;

    MIGRATOR.run(&pool).await?;
    Ok(pool)
}

/// Checkpoints the WAL into the main file and closes the pool.
///
/// Called on shutdown. Errors are logged and never panic.
pub async fn close(pool: &SqlitePool) {
    if let Err(e) = sqlx::query("PRAGMA wal_checkpoint(TRUNCATE)")
        .execute(pool)
        .await
    {
        tracing::warn!(error = %e, "WAL checkpoint failed during close");
    }
    pool.close().await;
}
