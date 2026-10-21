//! Injectable wall clock.
//!
//! Everything that reasons about "now" (scheduler catch-up, pause expiry,
//! relative time labels) takes a `Clock` so tests can drive time explicitly.
//! Wall-clock time is used on purpose: monotonic timers drift or stall across
//! Windows sleep/hibernate.

use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Duration;

use jiff::tz::TimeZone;
use jiff::{Timestamp, Zoned};

/// Source of the current wall-clock time and the user's time zone.
pub trait Clock: Send + Sync + 'static {
    /// Current instant.
    fn now(&self) -> Timestamp;

    /// Time zone used for user-facing calendar math ("until tomorrow").
    fn tz(&self) -> TimeZone;

    /// Current instant as unix epoch milliseconds (the DB timestamp unit).
    fn now_ms(&self) -> i64 {
        self.now().as_millisecond()
    }

    /// Current instant in the clock's time zone.
    fn now_zoned(&self) -> Zoned {
        self.now().to_zoned(self.tz())
    }
}

/// The real system clock with the OS time zone.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Timestamp {
        Timestamp::now()
    }

    fn tz(&self) -> TimeZone {
        TimeZone::system()
    }
}

/// A manually driven clock for tests. Always public so tests in other crates
/// (and the shell) can use it.
#[derive(Debug)]
pub struct FakeClock {
    ms: AtomicI64,
    tz: TimeZone,
}

impl FakeClock {
    /// Creates a fake clock at `start_ms` (unix epoch ms) in the IANA zone
    /// `tz_name` (for example `"Europe/Istanbul"` or `"UTC"`).
    ///
    /// # Panics
    /// Panics if `tz_name` is not a known IANA time zone. This is a test
    /// helper, so failing loudly is the right behaviour.
    pub fn new(start_ms: i64, tz_name: &str) -> Arc<FakeClock> {
        let tz = TimeZone::get(tz_name)
            .unwrap_or_else(|e| panic!("FakeClock: unknown time zone {tz_name:?}: {e}"));
        Arc::new(FakeClock {
            ms: AtomicI64::new(start_ms),
            tz,
        })
    }

    /// Jumps to an absolute instant (unix epoch ms).
    pub fn set_ms(&self, ms: i64) {
        self.ms.store(ms, Ordering::SeqCst);
    }

    /// Moves time forward by `d`.
    pub fn advance(&self, d: Duration) {
        let delta = i64::try_from(d.as_millis()).unwrap_or(i64::MAX);
        self.ms.fetch_add(delta, Ordering::SeqCst);
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Timestamp {
        let ms = self.ms.load(Ordering::SeqCst);
        Timestamp::from_millisecond(ms)
            .unwrap_or_else(|e| panic!("FakeClock: {ms} ms is out of range: {e}"))
    }

    fn tz(&self) -> TimeZone {
        self.tz.clone()
    }
}
