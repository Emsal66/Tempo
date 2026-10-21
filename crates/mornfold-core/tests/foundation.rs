//! Foundation tests for plan 01-01: schema, DB pragmas, settings, clock,
//! master key hygiene and the shared IPC contract serialization.

use std::time::Duration;

use mornfold_core::clock::{Clock, FakeClock};
use mornfold_core::db;
use mornfold_core::platform::MasterKey;
use mornfold_core::settings;
use mornfold_core::state::{
    Health, JobRunView, PauseChoice, PauseState, RunStatus, StatusSnapshot, UpdateState,
    VaultStatus,
};

const PHASE1_TABLES: [&str; 6] = [
    "job_runs",
    "jobs",
    "notifications",
    "secrets",
    "settings",
    "vault_meta",
];

async fn table_names(pool: &sqlx::SqlitePool) -> Vec<String> {
    sqlx::query_scalar(
        "SELECT name FROM sqlite_master WHERE type = 'table' \
         AND name NOT LIKE 'sqlite_%' AND name NOT LIKE '_sqlx_%' ORDER BY name",
    )
    .fetch_all(pool)
    .await
    .expect("list tables")
}

#[tokio::test]
async fn open_memory_creates_all_phase1_tables() {
    let pool = db::open_memory().await.expect("open memory db");
    assert_eq!(table_names(&pool).await, PHASE1_TABLES);

    let indexes: Vec<String> = sqlx::query_scalar(
        "SELECT name FROM sqlite_master WHERE type = 'index' AND name NOT LIKE 'sqlite_%' ORDER BY name",
    )
    .fetch_all(&pool)
    .await
    .expect("list indexes");
    assert_eq!(indexes, ["job_runs_started", "notifications_created"]);
}

#[tokio::test]
async fn open_file_db_uses_wal_and_foreign_keys() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("x.db");
    let pool = db::open(&path).await.expect("open file db");

    let mode: String = sqlx::query_scalar("PRAGMA journal_mode")
        .fetch_one(&pool)
        .await
        .expect("journal_mode");
    assert_eq!(mode.to_lowercase(), "wal");

    let fk: i64 = sqlx::query_scalar("PRAGMA foreign_keys")
        .fetch_one(&pool)
        .await
        .expect("foreign_keys");
    assert_eq!(fk, 1);

    assert_eq!(table_names(&pool).await, PHASE1_TABLES);

    db::close(&pool).await;
    assert!(pool.is_closed());

    // Re-opening an already migrated file is a no-op migration.
    let pool = db::open(&path).await.expect("re-open file db");
    assert_eq!(table_names(&pool).await, PHASE1_TABLES);
    db::close(&pool).await;
}

#[tokio::test]
async fn foreign_keys_cascade_job_runs() {
    let pool = db::open_memory().await.expect("open memory db");

    let orphan = sqlx::query(
        "INSERT INTO job_runs (id, job_id, scheduled_for, started_at, status) VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind("run-orphan")
    .bind("missing-job")
    .bind(0_i64)
    .bind(0_i64)
    .bind("Ok")
    .execute(&pool)
    .await;
    assert!(orphan.is_err(), "foreign key must reject an orphan run");

    sqlx::query("INSERT INTO jobs (id, kind, interval_sec, next_run_at) VALUES (?1, ?2, ?3, ?4)")
        .bind("heartbeat")
        .bind("heartbeat")
        .bind(300_i64)
        .bind(0_i64)
        .execute(&pool)
        .await
        .expect("insert job");
    sqlx::query(
        "INSERT INTO job_runs (id, job_id, scheduled_for, started_at, status) VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind("run-1")
    .bind("heartbeat")
    .bind(0_i64)
    .bind(0_i64)
    .bind("Ok")
    .execute(&pool)
    .await
    .expect("insert run");

    sqlx::query("DELETE FROM jobs WHERE id = ?1")
        .bind("heartbeat")
        .execute(&pool)
        .await
        .expect("delete job");
    let runs: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM job_runs")
        .fetch_one(&pool)
        .await
        .expect("count runs");
    assert_eq!(runs, 0, "ON DELETE CASCADE removes the job's runs");
}

#[tokio::test]
async fn settings_round_trip_missing_and_upsert() {
    let pool = db::open_memory().await.expect("open memory db");

    let missing: Option<PauseState> = settings::get(&pool, settings::KEY_PAUSE)
        .await
        .expect("get missing");
    assert_eq!(missing, None);

    let pause = PauseState {
        kind: PauseChoice::OneHour,
        since_ms: 1_000,
        until_ms: Some(3_601_000),
    };
    settings::set(&pool, settings::KEY_PAUSE, &pause, 1_000)
        .await
        .expect("set");
    let got: Option<PauseState> = settings::get(&pool, settings::KEY_PAUSE)
        .await
        .expect("get");
    assert_eq!(got, Some(pause));

    let indefinite = PauseState {
        kind: PauseChoice::Indefinite,
        since_ms: 2_000,
        until_ms: None,
    };
    settings::set(&pool, settings::KEY_PAUSE, &indefinite, 2_000)
        .await
        .expect("upsert");
    let rows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM settings WHERE key = ?1")
        .bind(settings::KEY_PAUSE)
        .fetch_one(&pool)
        .await
        .expect("count");
    assert_eq!(rows, 1, "upsert keeps a single row");
    let updated_at: i64 = sqlx::query_scalar("SELECT updated_at FROM settings WHERE key = ?1")
        .bind(settings::KEY_PAUSE)
        .fetch_one(&pool)
        .await
        .expect("updated_at");
    assert_eq!(updated_at, 2_000);
    let got: Option<PauseState> = settings::get(&pool, settings::KEY_PAUSE)
        .await
        .expect("get after upsert");
    assert_eq!(got, Some(indefinite));

    settings::set(&pool, settings::KEY_SOUND, &true, 3_000)
        .await
        .expect("set sound");
    settings::delete(&pool, settings::KEY_PAUSE)
        .await
        .expect("delete");
    let gone: Option<PauseState> = settings::get(&pool, settings::KEY_PAUSE)
        .await
        .expect("get deleted");
    assert_eq!(gone, None);
    let sound: Option<bool> = settings::get(&pool, settings::KEY_SOUND)
        .await
        .expect("get sound");
    assert_eq!(sound, Some(true), "delete only removes its own key");

    // Deleting a missing key is not an error.
    settings::delete(&pool, settings::KEY_SHOW_ON_NEXT_START)
        .await
        .expect("delete missing");
}

#[test]
fn fake_clock_advances_and_uses_its_time_zone() {
    // 2026-10-21T08:00:00Z
    let start_ms = 1_792_569_600_000;
    let clock = FakeClock::new(start_ms, "Europe/Istanbul");
    assert_eq!(clock.now_ms(), start_ms);

    clock.advance(Duration::from_secs(60));
    assert_eq!(clock.now_ms(), start_ms + 60_000);

    let zoned = clock.now_zoned();
    assert_eq!(zoned.time_zone().iana_name(), Some("Europe/Istanbul"));
    assert_eq!(zoned.timestamp().as_millisecond(), start_ms + 60_000);
    // Istanbul is UTC+3 all year: 08:01Z is 11:01 local.
    assert_eq!((zoned.hour(), zoned.minute()), (11, 1));

    clock.set_ms(0);
    assert_eq!(clock.now_ms(), 0);
    assert_eq!(clock.tz().iana_name(), Some("Europe/Istanbul"));
}

#[test]
fn master_key_debug_is_redacted_and_length_checked() {
    let bytes = [0xAB_u8; 32];
    let key = MasterKey::from_bytes(&bytes).expect("32 bytes");
    assert_eq!(key.as_bytes(), &bytes);

    let dbg = format!("{key:?}");
    assert!(dbg.contains("<redacted>"), "debug output: {dbg}");
    assert!(
        !dbg.to_lowercase().contains("ab"),
        "debug output leaks bytes: {dbg}"
    );
    assert!(!dbg.contains("171"), "debug output leaks bytes: {dbg}");

    assert!(MasterKey::from_bytes(&[0_u8; 31]).is_err());
    assert!(MasterKey::from_bytes(&[0_u8; 33]).is_err());

    let a = MasterKey::generate();
    let b = MasterKey::generate();
    assert_ne!(a.as_bytes(), b.as_bytes(), "generated keys must be random");
    assert_ne!(a.as_bytes(), &[0_u8; 32]);
}

#[test]
fn status_snapshot_serializes_camel_case() {
    let snapshot = StatusSnapshot {
        version: "0.1.0".into(),
        last_check_at_ms: Some(10),
        last_resume_at_ms: None,
        last_catch_up_count: 2,
        health: Health::Ok,
        pause: Some(PauseState {
            kind: PauseChoice::UntilTomorrow,
            since_ms: 5,
            until_ms: Some(50),
        }),
        update: UpdateState::UpToDate { checked_at_ms: 7 },
        vault: VaultStatus::Unlocked,
        unread: 3,
        recent_runs: vec![JobRunView {
            id: "r1".into(),
            job_id: "heartbeat".into(),
            scheduled_for_ms: 1,
            started_at_ms: 2,
            finished_at_ms: Some(3),
            status: RunStatus::Ok,
            catch_up: true,
            error: None,
        }],
    };

    let json = serde_json::to_value(&snapshot).expect("serialize");
    assert_eq!(json["lastCheckAtMs"], 10);
    assert!(json["lastResumeAtMs"].is_null());
    assert_eq!(json["lastCatchUpCount"], 2);
    assert_eq!(json["health"], "Ok");
    assert_eq!(json["pause"]["kind"], "UntilTomorrow");
    assert_eq!(json["pause"]["sinceMs"], 5);
    assert_eq!(json["pause"]["untilMs"], 50);
    assert_eq!(
        json["update"],
        serde_json::json!({"kind": "UpToDate", "checkedAtMs": 7})
    );
    assert_eq!(json["vault"], "Unlocked");
    assert_eq!(json["recentRuns"][0]["jobId"], "heartbeat");
    assert_eq!(json["recentRuns"][0]["scheduledForMs"], 1);
    assert_eq!(json["recentRuns"][0]["catchUp"], true);
    assert!(json.get("last_check_at_ms").is_none());

    let back: StatusSnapshot = serde_json::from_value(json).expect("deserialize");
    assert_eq!(back, snapshot);
}

#[test]
fn update_state_is_kind_tagged() {
    let ready = UpdateState::Ready {
        version: "0.2.0".into(),
    };
    assert_eq!(
        serde_json::to_value(&ready).expect("serialize"),
        serde_json::json!({"kind": "Ready", "version": "0.2.0"})
    );
    assert_eq!(
        serde_json::to_value(UpdateState::Idle).expect("serialize"),
        serde_json::json!({"kind": "Idle"})
    );
    assert_eq!(
        serde_json::to_value(UpdateState::Failed {
            message: "ağ yok".into()
        })
        .expect("serialize"),
        serde_json::json!({"kind": "Failed", "message": "ağ yok"})
    );
}
