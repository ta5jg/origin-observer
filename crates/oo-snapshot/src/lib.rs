// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-snapshot/src/lib.rs
// Purpose : Collect normalized and integrity-protected state snapshots.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Collect normalized and integrity-protected state snapshots.

pub mod collector;
pub mod integrity;
pub mod manifest;
pub mod normalization;
pub mod request;
pub mod snapshot;
pub mod validation;
