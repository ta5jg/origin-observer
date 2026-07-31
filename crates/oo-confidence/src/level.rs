// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-confidence/src/level.rs
// Purpose : Reconcile the workspace's confidence representations.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Reconcile the workspace's confidence representations.
//!
//! Three representations of confidence exist in this workspace, each earned
//! at a different layer, and this module is the one place that states how
//! they relate rather than leaving the relationship implicit:
//!
//! - [`oo_evidence::ReproductionStatus`] is a raw fact about one piece of
//!   evidence: was it observed, reproduced, independently verified, or did a
//!   later observation contradict it.
//! - `oo_config::WdrpConfidence` (`L0`–`L5`) is the constitutional gate from
//!   `WDRP.md`: the minimum reproduction status a *finding* must reach before
//!   it may be published as accepted project knowledge. This crate does not
//!   depend on `oo-config` to avoid coupling a domain crate to the
//!   configuration layer, so the `L0`–`L5` codes are reproduced here as
//!   strings rather than imported as a type; [`to_wdrp_code`] is the single
//!   function responsible for keeping that reproduction correct.
//! - [`oo_model::ConfidenceLevel`] is a general seven-point qualitative scale
//!   usable for *any* assessment, not only reproduction status — a proxy
//!   resolution's confidence or a metadata match's confidence can use it too.
//!
//! A [`oo_evidence::ReproductionStatus::Contradicted`] finding has no honest
//! place on either the WDRP scale or the general scale: it is not merely
//! "unknown," it is actively refuted. Both conversions below return `None`
//! for it rather than silently mapping it to the lowest level, which would
//! make a refuted claim look identical to one nobody has looked at yet.

use oo_evidence::ReproductionStatus;
use oo_model::confidence::ConfidenceLevel;

/// Converts a reproduction status to the general confidence scale.
///
/// Returns `None` for [`ReproductionStatus::Contradicted`]: see the module
/// documentation for why a refuted claim is not representable as a level.
#[must_use]
pub const fn to_confidence_level(status: ReproductionStatus) -> Option<ConfidenceLevel> {
    match status {
        ReproductionStatus::Unknown => Some(ConfidenceLevel::None),
        ReproductionStatus::Observed => Some(ConfidenceLevel::Low),
        ReproductionStatus::Reproduced => Some(ConfidenceLevel::High),
        ReproductionStatus::IndependentlyVerified => Some(ConfidenceLevel::Certain),
        ReproductionStatus::Contradicted => None,
    }
}

/// Converts a reproduction status to its WDRP confidence code (`"L0"`–`"L5"`).
///
/// Returns `None` for [`ReproductionStatus::Contradicted`], for the same
/// reason as [`to_confidence_level`].
#[must_use]
pub const fn to_wdrp_code(status: ReproductionStatus) -> Option<&'static str> {
    match status {
        ReproductionStatus::Unknown => Some("L0"),
        ReproductionStatus::Observed => Some("L2"),
        ReproductionStatus::Reproduced => Some("L3"),
        ReproductionStatus::IndependentlyVerified => Some("L5"),
        ReproductionStatus::Contradicted => None,
    }
}

/// Returns whether a reproduction status meets WDRP's publication bar: only
/// `L5` (independently verified) may become accepted project knowledge.
#[must_use]
pub const fn is_publishable(status: ReproductionStatus) -> bool {
    matches!(status, ReproductionStatus::IndependentlyVerified)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_positive_status_maps_to_a_confidence_level() {
        assert_eq!(
            to_confidence_level(ReproductionStatus::Unknown),
            Some(ConfidenceLevel::None)
        );
        assert_eq!(
            to_confidence_level(ReproductionStatus::Observed),
            Some(ConfidenceLevel::Low)
        );
        assert_eq!(
            to_confidence_level(ReproductionStatus::Reproduced),
            Some(ConfidenceLevel::High)
        );
        assert_eq!(
            to_confidence_level(ReproductionStatus::IndependentlyVerified),
            Some(ConfidenceLevel::Certain)
        );
    }

    #[test]
    fn a_contradicted_status_has_no_confidence_level() {
        assert_eq!(to_confidence_level(ReproductionStatus::Contradicted), None);
    }

    #[test]
    fn wdrp_codes_match_the_constitutions_six_levels() {
        assert_eq!(to_wdrp_code(ReproductionStatus::Unknown), Some("L0"));
        assert_eq!(
            to_wdrp_code(ReproductionStatus::IndependentlyVerified),
            Some("L5")
        );
    }

    #[test]
    fn a_contradicted_status_has_no_wdrp_code() {
        assert_eq!(to_wdrp_code(ReproductionStatus::Contradicted), None);
    }

    #[test]
    fn only_independently_verified_is_publishable() {
        assert!(is_publishable(ReproductionStatus::IndependentlyVerified));
        assert!(!is_publishable(ReproductionStatus::Reproduced));
        assert!(!is_publishable(ReproductionStatus::Contradicted));
    }
}
