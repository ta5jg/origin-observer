// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-config/src/lib.rs
// Purpose : Load and validate Origin Observer configuration.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Load and validate Origin Observer configuration.

pub mod environment;
pub mod error;
pub mod loader;
pub mod model;
pub mod validation;

pub use environment::{Credentials, EnvironmentOverrides, Secret};
pub use error::{ConfigError, ConfigResult};
pub use loader::{load_from_directory, ConfigProvenance, ConfigSource, LoadedConfig};
pub use model::{
    ChainConfig, ChainFamily, ChainKind, Config, ProviderConfig, ProviderKind, WalletConfig,
    WalletPlatform, WdrpConfidence,
};
