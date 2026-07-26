// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-rpc/src/lib.rs
// Purpose : Perform deterministic and attributable JSON-RPC communication.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Perform deterministic and attributable JSON-RPC communication.

pub mod batch;
pub mod client;
pub mod endpoint;
pub mod error;
pub mod fixture;
pub mod request;
pub mod response;
pub mod retry;
pub mod trace;
pub mod transport;
