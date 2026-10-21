//! Mornfold domain core.
//!
//! Contains all business logic that does not need Tauri, so it builds and
//! tests on Linux. The `src-tauri` shell implements the platform traits in
//! [`platform`] and translates between Tauri/Win32 objects and core values.
//!
//! Module ownership (Phase 1): plan 01-01 owns `clock`, `db`, `error`,
//! `platform`, `settings` and `state`; the remaining modules are filled in by
//! the wave-2 plans named in their module docs.

#![forbid(unsafe_code)]

pub mod clock;
pub mod db;
pub mod error;
pub mod platform;
pub mod settings;
pub mod state;

pub mod i18n;
pub mod masking;
pub mod notify;
pub mod pause;
pub mod relative_time;
pub mod scheduler;
pub mod startup;
pub mod tray_model;
pub mod vault;

pub use error::CoreError;
