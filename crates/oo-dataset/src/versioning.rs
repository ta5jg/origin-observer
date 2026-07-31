// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-dataset/src/versioning.rs
// Purpose : Track dataset schema versions and their compatibility.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Track dataset schema versions and their compatibility.
//!
//! A minor version bump adds fields without removing or repurposing existing
//! ones, so a reader built for an older minor version can still read newer
//! data. A major version bump makes no such promise. Compatibility is
//! therefore major-version equality, not a full ordering comparison.

/// A dataset schema version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DatasetVersion {
    major: u32,
    minor: u32,
}

impl DatasetVersion {
    /// Creates a dataset version.
    #[must_use]
    pub const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    /// Returns the major version.
    #[must_use]
    pub const fn major(self) -> u32 {
        self.major
    }

    /// Returns the minor version.
    #[must_use]
    pub const fn minor(self) -> u32 {
        self.minor
    }

    /// Returns whether a reader built for this version can read data written
    /// at `other`'s version: they must share a major version.
    #[must_use]
    pub const fn is_compatible_with(self, other: Self) -> bool {
        self.major == other.major
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_sharing_a_major_version_are_compatible() {
        assert!(DatasetVersion::new(1, 0).is_compatible_with(DatasetVersion::new(1, 3)));
    }

    #[test]
    fn versions_with_different_major_versions_are_incompatible() {
        assert!(!DatasetVersion::new(1, 3).is_compatible_with(DatasetVersion::new(2, 0)));
    }

    #[test]
    fn a_higher_minor_version_orders_after_a_lower_one() {
        assert!(DatasetVersion::new(1, 3) > DatasetVersion::new(1, 2));
    }
}
