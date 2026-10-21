//! Key/value settings stored as JSON in the `settings` table.
//!
//! All SQL is static (`&'static str`) with binds only.

use serde::Serialize;
use serde::de::DeserializeOwned;
use sqlx::SqlitePool;

use crate::error::CoreError;

/// Active pause (`state::PauseState`), absent when not paused.
pub const KEY_PAUSE: &str = "pause";
/// Whether toasts play the Mornfold sound.
pub const KEY_SOUND: &str = "sound";
/// Show the main window on the next start even if launched minimized.
pub const KEY_SHOW_ON_NEXT_START: &str = "show_on_next_start";

/// Reads and deserializes a setting. Returns `Ok(None)` when the key is absent.
pub async fn get<T: DeserializeOwned>(
    pool: &SqlitePool,
    key: &str,
) -> Result<Option<T>, CoreError> {
    let raw: Option<String> = sqlx::query_scalar("SELECT value FROM settings WHERE key = ?1")
        .bind(key)
        .fetch_optional(pool)
        .await?;
    match raw {
        Some(json) => Ok(Some(serde_json::from_str(&json)?)),
        None => Ok(None),
    }
}

/// Serializes and upserts a setting.
pub async fn set<T: Serialize>(
    pool: &SqlitePool,
    key: &str,
    value: &T,
    now_ms: i64,
) -> Result<(), CoreError> {
    let json = serde_json::to_string(value)?;
    sqlx::query(
        "INSERT INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3) \
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
    )
    .bind(key)
    .bind(json)
    .bind(now_ms)
    .execute(pool)
    .await?;
    Ok(())
}

/// Removes a setting. Deleting a missing key is not an error.
pub async fn delete(pool: &SqlitePool, key: &str) -> Result<(), CoreError> {
    sqlx::query("DELETE FROM settings WHERE key = ?1")
        .bind(key)
        .execute(pool)
        .await?;
    Ok(())
}
