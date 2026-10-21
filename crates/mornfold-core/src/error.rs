//! Core error type shared by the DB, settings and later core modules.

/// Errors returned by `mornfold-core` foundation APIs.
///
/// Messages are Turkish because they can surface in the UI; the variant names
/// stay English for code readers.
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("veritabanı hatası: {0}")]
    Db(#[from] sqlx::Error),
    #[error("migrasyon hatası: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error("serileştirme hatası: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0}")]
    Invalid(String),
}
