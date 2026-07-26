// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-core/src/lib.rs
// Purpose : Public API for the Origin Observer core crate.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Origin Observer Core
//!
//! `oo-core` contains the fundamental building blocks shared by every crate in
//! the Origin Observer workspace.
//!
//! The crate intentionally contains only infrastructure:
//!
//! - identifiers
//! - runtime
//! - execution context
//! - clocks
//! - digest primitives
//! - serialization traits
//! - common errors
//! - common result types
//!
//! Domain models belong in `oo-model`.
//! Blockchain implementations belong in provider crates.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub mod clock;
pub mod context;
pub mod digest;
pub mod error;
pub mod id;
pub mod result;
pub mod runtime;
pub mod serialization;

// -----------------------------------------------------------------------------
// Re-exports
// -----------------------------------------------------------------------------

pub use clock::{Clock, ManualClock, SystemClock};

pub use context::{Context, SharedContext};

pub use digest::{Digest, DigestParseError, SHA256_LENGTH};

pub use error::{BoxError, Error, ErrorKind};

pub use id::{
    AddressId, AssetId, BlockId, BlockchainId, ContractId, EvidenceId, ExecutionId, ExperimentId,
    Identifier, NetworkId, ProviderId, ReportId, RuntimeId, SessionId, SnapshotId, TransactionId,
    WalletId, WorkspaceId,
};

pub use result::{done, failure, success, Result, ResultExt};

pub use runtime::Runtime;

pub use serialization::{
    bytes_equal, bytes_to_text, clone_bytes, deserialize, serialize, text_to_bytes, BinaryBlob,
    BinaryDeserializable, BinarySerializable, TextDeserializable, TextSerializable,
};
