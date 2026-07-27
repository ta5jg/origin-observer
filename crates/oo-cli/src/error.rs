// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-cli/src/error.rs
// Purpose : Implement the error module for oo-cli.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Implements the error module for oo-cli.

/// CLI result type.
pub type CliResult<T> = anyhow::Result<T>;
