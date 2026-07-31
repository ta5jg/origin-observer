// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/version.rs
// Purpose : Parse wallet client versions.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Parse wallet client versions.
//!
//! Discovery behavior changes between wallet releases, so an observation that
//! does not record which version answered cannot be compared against another
//! run. Parsing uses the `semver` crate rather than a hand-rolled parser: a
//! wallet version string is exactly what semantic versioning describes, and a
//! subtly wrong hand-rolled comparison would misorder versions silently.

use semver::Version;

/// A wallet client version, keeping both the raw string and, when it parses,
/// a comparable semantic version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletVersion {
    /// Exactly as reported.
    pub raw: String,
    /// Parsed semantic version, when the raw string is well-formed semver.
    pub parsed: Option<Version>,
}

impl WalletVersion {
    /// Parses a version string.
    ///
    /// Many wallets report a bare `major.minor.patch` without a `v` prefix;
    /// a leading `v` is stripped before parsing since it is not part of
    /// semver itself.
    #[must_use]
    pub fn parse(raw: impl Into<String>) -> Self {
        let raw = raw.into();
        let candidate = raw.strip_prefix('v').unwrap_or(&raw);
        Self {
            parsed: Version::parse(candidate).ok(),
            raw,
        }
    }

    /// Returns whether this version is strictly older than another.
    ///
    /// Returns `None` when either version failed to parse: an unparseable
    /// version cannot be honestly compared, and treating it as "equal" or
    /// "oldest" would fabricate an ordering that was never observed.
    #[must_use]
    pub fn is_older_than(&self, other: &Self) -> Option<bool> {
        Some(self.parsed.as_ref()? < other.parsed.as_ref()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_well_formed_version_parses() {
        let version = WalletVersion::parse("11.7.2");
        assert!(version.parsed.is_some());
        assert_eq!(version.raw, "11.7.2");
    }

    #[test]
    fn a_leading_v_is_stripped_before_parsing() {
        let version = WalletVersion::parse("v11.7.2");
        assert!(version.parsed.is_some());
        assert_eq!(
            version.raw, "v11.7.2",
            "the raw string is preserved verbatim"
        );
    }

    #[test]
    fn a_malformed_version_does_not_parse_but_keeps_its_raw_text() {
        let version = WalletVersion::parse("release-candidate-3");
        assert!(version.parsed.is_none());
        assert_eq!(version.raw, "release-candidate-3");
    }

    #[test]
    fn comparison_orders_parsed_versions() {
        let old = WalletVersion::parse("11.0.0");
        let new = WalletVersion::parse("11.7.2");
        assert_eq!(old.is_older_than(&new), Some(true));
        assert_eq!(new.is_older_than(&old), Some(false));
    }

    #[test]
    fn comparison_is_honest_about_unparseable_input() {
        let unknown = WalletVersion::parse("unknown");
        let known = WalletVersion::parse("1.0.0");
        assert_eq!(unknown.is_older_than(&known), None);
    }
}
