// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/model.rs
// Purpose : Proxy classification and resolution result model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Proxy classification and resolution result model.

/// Classification of a proxy architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProxyKind {
    /// EIP-1167 minimal proxy: a fixed 45-byte template delegating to an
    /// address embedded directly in the bytecode.
    Eip1167Minimal,
    /// EIP-1967 transparent upgradeable proxy: implementation and admin
    /// slots both in use.
    Eip1967Transparent,
    /// EIP-1967 storage layout with no admin slot in use, consistent with a
    /// UUPS proxy whose upgrade logic lives in the implementation itself.
    Eip1967Uups,
    /// EIP-1967 beacon proxy: the beacon slot names a contract that itself
    /// resolves the implementation address.
    Eip1967Beacon,
    /// Pre-1967 OpenZeppelin transparent proxy storage layout.
    LegacyOpenZeppelinTransparent,
    /// Responds to ERC-165 `supportsInterface` for the diamond loupe
    /// interface, consistent with EIP-2535.
    Diamond,
    /// No recognized proxy pattern was detected.
    ///
    /// This does not mean the contract is not a proxy: it means none of the
    /// patterns this crate checks matched. A custom or non-standard proxy
    /// would also classify here.
    Unknown,
}

impl ProxyKind {
    /// Returns whether this classification identifies a specific implication
    /// address directly, without a further follow-up call.
    #[must_use]
    pub const fn resolves_directly(self) -> bool {
        !matches!(self, Self::Eip1967Beacon | Self::Unknown)
    }
}

/// One piece of evidence a detection step observed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyEvidence {
    /// What was checked, in plain language.
    pub check: String,
    /// What was observed.
    pub observation: String,
}

impl ProxyEvidence {
    /// Records one evidence entry.
    #[must_use]
    pub fn new(check: impl Into<String>, observation: impl Into<String>) -> Self {
        Self {
            check: check.into(),
            observation: observation.into(),
        }
    }
}

/// Result of resolving a contract's proxy architecture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyResolution {
    /// Classified proxy kind.
    pub kind: ProxyKind,
    /// Resolved implementation address, when one was determined directly.
    ///
    /// For [`ProxyKind::Eip1967Beacon`], this is the beacon's own address
    /// unless a follow-up call to the beacon's `implementation()` also
    /// succeeded, in which case it is the final implementation.
    pub implementation: Option<[u8; 20]>,
    /// Resolved admin address, when the detected pattern has one.
    pub admin: Option<[u8; 20]>,
    /// Every check performed and what it observed, in the order performed.
    ///
    /// This is the resolution's audit trail: a caller citing "this is a
    /// transparent proxy" as a finding must be able to show what was checked,
    /// not only the conclusion.
    pub evidence: Vec<ProxyEvidence>,
}

impl ProxyResolution {
    /// Creates an unresolved result with no evidence yet recorded.
    #[must_use]
    pub const fn unknown() -> Self {
        Self {
            kind: ProxyKind::Unknown,
            implementation: None,
            admin: None,
            evidence: Vec::new(),
        }
    }

    /// Records one evidence entry.
    pub fn record(&mut self, check: impl Into<String>, observation: impl Into<String>) {
        self.evidence.push(ProxyEvidence::new(check, observation));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_beacon_and_unknown_require_a_follow_up_to_resolve() {
        assert!(ProxyKind::Eip1167Minimal.resolves_directly());
        assert!(ProxyKind::Eip1967Transparent.resolves_directly());
        assert!(!ProxyKind::Eip1967Beacon.resolves_directly());
        assert!(!ProxyKind::Unknown.resolves_directly());
    }

    #[test]
    fn an_unknown_resolution_carries_no_addresses() {
        let resolution = ProxyResolution::unknown();
        assert_eq!(resolution.kind, ProxyKind::Unknown);
        assert!(resolution.implementation.is_none());
        assert!(resolution.evidence.is_empty());
    }

    #[test]
    fn evidence_records_in_order() {
        let mut resolution = ProxyResolution::unknown();
        resolution.record("bytecode length", "45 bytes");
        resolution.record("eip1167 template", "matched");
        assert_eq!(resolution.evidence.len(), 2);
        assert_eq!(resolution.evidence[0].check, "bytecode length");
    }
}
