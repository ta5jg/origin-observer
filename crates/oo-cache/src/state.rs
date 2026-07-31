// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cache/src/state.rs
// Purpose : A change in observed cache state between two observations.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! A change in observed cache state between two observations.

use oo_model::cache::CacheState;

/// A transition from one observed cache state to another.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheTransition {
    before: CacheState,
    after: CacheState,
}

impl CacheTransition {
    /// Creates a transition between two observed states.
    #[must_use]
    pub const fn new(before: CacheState, after: CacheState) -> Self {
        Self { before, after }
    }

    /// Returns the state before the transition.
    #[must_use]
    pub const fn before(self) -> CacheState {
        self.before
    }

    /// Returns the state after the transition.
    #[must_use]
    pub const fn after(self) -> CacheState {
        self.after
    }

    /// Returns whether this transition represents a successful invalidation:
    /// a cache that held a value (warm or stale) that no longer does (empty
    /// or explicitly invalidated).
    #[must_use]
    pub const fn is_invalidation(self) -> bool {
        matches!(self.before, CacheState::Warm | CacheState::Stale)
            && matches!(self.after, CacheState::Empty | CacheState::Invalidated)
    }

    /// Returns whether the observed state did not change.
    #[must_use]
    pub fn is_unchanged(self) -> bool {
        self.before == self.after
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warm_to_empty_is_an_invalidation() {
        assert!(CacheTransition::new(CacheState::Warm, CacheState::Empty).is_invalidation());
    }

    #[test]
    fn stale_to_invalidated_is_an_invalidation() {
        assert!(CacheTransition::new(CacheState::Stale, CacheState::Invalidated).is_invalidation());
    }

    #[test]
    fn warm_to_warm_is_not_an_invalidation() {
        assert!(!CacheTransition::new(CacheState::Warm, CacheState::Warm).is_invalidation());
        assert!(CacheTransition::new(CacheState::Warm, CacheState::Warm).is_unchanged());
    }

    #[test]
    fn empty_to_warm_is_not_an_invalidation() {
        assert!(!CacheTransition::new(CacheState::Empty, CacheState::Warm).is_invalidation());
    }
}
