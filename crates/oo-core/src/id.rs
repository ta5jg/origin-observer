// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-core/src/id.rs
// Purpose : Strongly typed identifiers shared across Origin Observer.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Strongly typed identifiers used throughout Origin Observer.
//!
//! The goal of this module is to eliminate accidental mixing of identifiers
//! belonging to different domains while keeping the implementation lightweight.

use core::fmt;
use core::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Common behavior implemented by every strongly typed identifier.
pub trait Identifier: Clone + Eq + Ord + core::hash::Hash + fmt::Debug + fmt::Display {
    /// Returns the underlying UUID.
    fn uuid(&self) -> Uuid;
}

/// Macro generating strongly typed UUID wrappers.
macro_rules! define_identifier {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            /// Creates a new random identifier.
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            /// Creates an identifier from an existing UUID.
            #[must_use]
            pub const fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            /// Returns the wrapped UUID.
            #[must_use]
            pub const fn into_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Identifier for $name {
            fn uuid(&self) -> Uuid {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<Uuid> for $name {
            fn from(value: Uuid) -> Self {
                Self(value)
            }
        }

        impl From<$name> for Uuid {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Ok(Self(Uuid::parse_str(value)?))
            }
        }
    };
}

define_identifier!(WorkspaceId);
define_identifier!(CacheId);
define_identifier!(DiscoveryId);
define_identifier!(MetadataId);
define_identifier!(RuntimeId);
define_identifier!(SessionId);
define_identifier!(ExecutionId);
define_identifier!(GraphId);

define_identifier!(BlockchainId);
define_identifier!(NetworkId);
define_identifier!(ProviderId);

define_identifier!(WalletId);
define_identifier!(AddressId);
define_identifier!(ContractId);
define_identifier!(AssetId);

define_identifier!(BlockId);
define_identifier!(TransactionId);
define_identifier!(SnapshotId);

define_identifier!(EvidenceId);
define_identifier!(ConfidenceId);
define_identifier!(ReportId);
define_identifier!(ExperimentId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_are_unique() {
        let a = WalletId::new();
        let b = WalletId::new();

        assert_ne!(a, b);
    }

    #[test]
    fn identifier_roundtrip_uuid() {
        let original = WalletId::new();

        let uuid: Uuid = original.into();

        let restored = WalletId::from(uuid);

        assert_eq!(original, restored);
    }

    #[test]
    fn identifier_roundtrip_string() {
        let original = ContractId::new();

        let text = original.to_string();

        let parsed = ContractId::from_str(&text).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn different_identifier_types_cannot_mix() {
        let wallet = WalletId::new();
        let contract = ContractId::new();

        assert_ne!(wallet.uuid(), contract.uuid());
    }

    #[test]
    fn an_identifier_serializes_as_its_bare_uuid_string() {
        let id = WalletId::new();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, format!("\"{}\"", id));
    }

    #[test]
    fn an_identifier_round_trips_through_json() {
        let original = WalletId::new();
        let json = serde_json::to_string(&original).unwrap();
        let restored: WalletId = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }
}
