// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-core/src/clock.rs
// Purpose : Time abstractions shared across Origin Observer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Clock abstractions used throughout Origin Observer.
//!
//! The purpose of this module is to provide deterministic and testable access
//! to time. Production code should depend on the [`Clock`] trait instead of
//! directly calling `SystemTime::now()`.

use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Common interface implemented by all clocks.
pub trait Clock: Send + Sync {
    /// Returns the current system time.
    fn now(&self) -> SystemTime;

    /// Returns the current UNIX timestamp in milliseconds.
    fn unix_millis(&self) -> u128 {
        self.now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_millis()
    }

    /// Returns the current UNIX timestamp in seconds.
    fn unix_seconds(&self) -> u64 {
        self.now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before unix epoch")
            .as_secs()
    }
}

/// Production clock backed by the operating system.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

/// Deterministic clock intended for tests.
#[derive(Clone, Debug)]
pub struct ManualClock {
    current: Arc<Mutex<SystemTime>>,
}

impl ManualClock {
    /// Creates a new manual clock.
    #[must_use]
    pub fn new(initial: SystemTime) -> Self {
        Self {
            current: Arc::new(Mutex::new(initial)),
        }
    }

    /// Creates a manual clock positioned at the UNIX epoch.
    #[must_use]
    pub fn from_unix_epoch() -> Self {
        Self::new(UNIX_EPOCH)
    }

    /// Sets the current time.
    pub fn set(&self, instant: SystemTime) {
        *self.current.lock().unwrap() = instant;
    }

    /// Advances the clock.
    pub fn advance(&self, duration: Duration) {
        let mut guard = self.current.lock().unwrap();
        *guard += duration;
    }

    /// Rewinds the clock.
    pub fn rewind(&self, duration: Duration) {
        let mut guard = self.current.lock().unwrap();
        *guard -= duration;
    }
}

impl Clock for ManualClock {
    fn now(&self) -> SystemTime {
        *self.current.lock().unwrap()
    }
}

/// Returns the duration elapsed between two timestamps.
#[must_use]
pub fn elapsed(start: SystemTime, end: SystemTime) -> Duration {
    end.duration_since(start).unwrap_or(Duration::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_clock_returns_time() {
        let clock = SystemClock;

        assert!(clock.unix_seconds() > 0);
    }

    #[test]
    fn manual_clock_can_advance() {
        let clock = ManualClock::from_unix_epoch();

        assert_eq!(clock.unix_seconds(), 0);

        clock.advance(Duration::from_secs(10));

        assert_eq!(clock.unix_seconds(), 10);
    }

    #[test]
    fn manual_clock_can_rewind() {
        let clock = ManualClock::new(UNIX_EPOCH + Duration::from_secs(30));

        clock.rewind(Duration::from_secs(5));

        assert_eq!(clock.unix_seconds(), 25);
    }

    #[test]
    fn elapsed_duration() {
        let start = UNIX_EPOCH;
        let end = UNIX_EPOCH + Duration::from_secs(42);

        assert_eq!(elapsed(start, end), Duration::from_secs(42));
    }
}
