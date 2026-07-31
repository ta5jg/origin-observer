// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-utils/src/lib.rs
// Purpose : Provide small dependency-light reusable utilities.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Provide small dependency-light reusable utilities.

pub mod error;
pub mod fs;
pub mod hash;
pub mod text;
pub mod validation;

pub use error::{UtilsError, UtilsResult};
pub use hash::{Digest, DIGEST_ALGORITHM};
