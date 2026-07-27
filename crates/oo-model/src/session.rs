// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/session.rs
// Purpose : Observation session domain model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Observation session domain model.
//!
//! A session represents a complete execution of the Origin Observer engine.
//! During a session one or more providers are queried, evidence is collected,
//! snapshots are produced and execution statistics are accumulated.

use std::collections::BTreeSet;

use oo_core::error::invalid_argument;
use oo_core::{ProviderId, Result, SessionId, SnapshotId};

/// Maximum session name length.
pub const MAX_SESSION_NAME_LENGTH: usize = 128;

/// Session kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum SessionKind {
    #[default]
    Unknown,

    Manual,

    Scheduled,

    Background,

    Continuous,

    Verification,

    Recovery,

    Benchmark,
}

/// Session lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum SessionStatus {
    Created,

    Running,

    Completed,

    Failed,

    Cancelled,

    #[default]
    Unknown,
}

/// Observation session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    id: SessionId,

    name: String,

    kind: SessionKind,

    status: SessionStatus,

    started_unix_ms: Option<u128>,

    finished_unix_ms: Option<u128>,

    providers: BTreeSet<ProviderId>,

    snapshots: BTreeSet<SnapshotId>,

    warning_count: u64,

    error_count: u64,
}

impl Session {
    /// Creates a new session.
    pub fn new(name: impl Into<String>) -> Result<Self> {
        let name = normalize(name.into())?;

        if name.len() > MAX_SESSION_NAME_LENGTH {
            return Err(invalid_argument("session name is too long"));
        }

        Ok(Self {
            id: SessionId::new(),
            name,
            kind: SessionKind::Unknown,
            status: SessionStatus::Created,
            started_unix_ms: None,
            finished_unix_ms: None,
            providers: BTreeSet::new(),
            snapshots: BTreeSet::new(),
            warning_count: 0,
            error_count: 0,
        })
    }

    #[must_use]
    pub const fn id(&self) -> SessionId {
        self.id
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, value: impl Into<String>) -> Result<()> {
        let value = normalize(value.into())?;

        if value.len() > MAX_SESSION_NAME_LENGTH {
            return Err(invalid_argument("session name is too long"));
        }

        self.name = value;

        Ok(())
    }

    #[must_use]
    pub const fn kind(&self) -> SessionKind {
        self.kind
    }

    pub const fn set_kind(&mut self, kind: SessionKind) {
        self.kind = kind;
    }

    #[must_use]
    pub const fn status(&self) -> SessionStatus {
        self.status
    }

    pub const fn start(&mut self, unix_ms: u128) {
        self.started_unix_ms = Some(unix_ms);
        self.status = SessionStatus::Running;
    }

    pub const fn finish(&mut self, unix_ms: u128) {
        self.finished_unix_ms = Some(unix_ms);
        self.status = SessionStatus::Completed;
    }

    pub const fn fail(&mut self, unix_ms: u128) {
        self.finished_unix_ms = Some(unix_ms);
        self.status = SessionStatus::Failed;
    }

    pub const fn cancel(&mut self, unix_ms: u128) {
        self.finished_unix_ms = Some(unix_ms);
        self.status = SessionStatus::Cancelled;
    }

    #[must_use]
    pub const fn started_unix_ms(&self) -> Option<u128> {
        self.started_unix_ms
    }

    #[must_use]
    pub const fn finished_unix_ms(&self) -> Option<u128> {
        self.finished_unix_ms
    }

    pub fn add_provider(&mut self, id: ProviderId) -> bool {
        self.providers.insert(id)
    }

    pub fn remove_provider(&mut self, id: ProviderId) -> bool {
        self.providers.remove(&id)
    }

    #[must_use]
    pub fn providers(&self) -> &BTreeSet<ProviderId> {
        &self.providers
    }

    pub fn add_snapshot(&mut self, id: SnapshotId) -> bool {
        self.snapshots.insert(id)
    }

    pub fn remove_snapshot(&mut self, id: SnapshotId) -> bool {
        self.snapshots.remove(&id)
    }

    #[must_use]
    pub fn snapshots(&self) -> &BTreeSet<SnapshotId> {
        &self.snapshots
    }

    pub fn increment_warning(&mut self) {
        self.warning_count += 1;
    }

    pub fn increment_error(&mut self) {
        self.error_count += 1;
    }

    #[must_use]
    pub const fn warning_count(&self) -> u64 {
        self.warning_count
    }

    #[must_use]
    pub const fn error_count(&self) -> u64 {
        self.error_count
    }

    #[must_use]
    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }

    #[must_use]
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    #[must_use]
    pub const fn is_running(&self) -> bool {
        matches!(self.status, SessionStatus::Running)
    }

    #[must_use]
    pub const fn is_finished(&self) -> bool {
        matches!(
            self.status,
            SessionStatus::Completed | SessionStatus::Failed | SessionStatus::Cancelled
        )
    }
}

fn normalize(value: String) -> Result<String> {
    let value = value.trim().to_owned();

    if value.is_empty() {
        return Err(invalid_argument("value must not be empty"));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_argument("value contains control characters"));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn session() -> Session {
        Session::new("Genesis").unwrap()
    }

    #[test]
    fn unique_ids() {
        let a = session();
        let b = session();

        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn lifecycle() {
        let mut s = session();

        s.start(100);

        assert!(s.is_running());

        s.finish(200);

        assert!(s.is_finished());

        assert_eq!(s.finished_unix_ms(), Some(200),);
    }

    #[test]
    fn provider_management() {
        let mut s = session();

        let id = ProviderId::new();

        assert!(s.add_provider(id));

        assert_eq!(s.provider_count(), 1,);

        assert!(s.remove_provider(id));

        assert_eq!(s.provider_count(), 0,);
    }

    #[test]
    fn snapshot_management() {
        let mut s = session();

        let id = SnapshotId::new();

        assert!(s.add_snapshot(id));

        assert_eq!(s.snapshot_count(), 1,);

        assert!(s.remove_snapshot(id));

        assert_eq!(s.snapshot_count(), 0,);
    }

    #[test]
    fn counters() {
        let mut s = session();

        s.increment_warning();
        s.increment_warning();
        s.increment_error();

        assert_eq!(s.warning_count(), 2,);

        assert_eq!(s.error_count(), 1,);
    }

    #[test]
    fn rename() {
        let mut s = session();

        s.set_name("Production").unwrap();

        assert_eq!(s.name(), "Production",);
    }

    #[test]
    fn empty_name_rejected() {
        assert!(Session::new("").is_err());
    }
}
