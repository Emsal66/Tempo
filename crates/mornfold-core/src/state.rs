//! Shared IPC / status contract types.
//!
//! These types cross the Rust → TypeScript boundary (via tauri-specta) and are
//! consumed verbatim by the wave-2 plans. Do not rename fields or variants
//! without updating every consumer.
//!
//! Timestamps are unix epoch milliseconds (`*_ms`).

use serde::{Deserialize, Serialize};

/// How long the user paused background work (D-13).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum PauseChoice {
    OneHour,
    UntilTomorrow,
    Indefinite,
}

/// Active pause. `until_ms` is `None` for `Indefinite`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PauseState {
    pub kind: PauseChoice,
    pub since_ms: i64,
    pub until_ms: Option<i64>,
}

/// Overall background health, reflected by the tray icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum Health {
    Ok,
    Degraded,
    Error,
}

/// Auto-update state machine as shown in the status screen and tray.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind")]
pub enum UpdateState {
    Idle,
    Checking,
    Downloading,
    UpToDate {
        #[serde(rename = "checkedAtMs")]
        checked_at_ms: i64,
    },
    Ready {
        version: String,
    },
    Failed {
        message: String,
    },
}

/// Vault (secret store) availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum VaultStatus {
    Unlocked,
    KeyLost,
    KeyMismatch,
    Unavailable,
}

/// Outcome of a single scheduler job run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum RunStatus {
    Running,
    Ok,
    Failed,
    TimedOut,
}

/// A `job_runs` row as shown in the status screen.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct JobRunView {
    pub id: String,
    pub job_id: String,
    pub scheduled_for_ms: i64,
    pub started_at_ms: i64,
    pub finished_at_ms: Option<i64>,
    pub status: RunStatus,
    pub catch_up: bool,
    pub error: Option<String>,
}

/// Everything the status screen and tray need in one snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct StatusSnapshot {
    pub version: String,
    pub last_check_at_ms: Option<i64>,
    pub last_resume_at_ms: Option<i64>,
    pub last_catch_up_count: u32,
    pub health: Health,
    pub pause: Option<PauseState>,
    pub update: UpdateState,
    pub vault: VaultStatus,
    pub unread: u32,
    pub recent_runs: Vec<JobRunView>,
}
