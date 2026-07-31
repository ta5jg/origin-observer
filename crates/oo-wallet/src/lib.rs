// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-wallet/src/lib.rs
// Purpose : Model wallet-specific discovery observations and adapters.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Model wallet-specific discovery observations and adapters.

pub mod adapter;
pub mod cache;
pub mod capability;
pub mod coinbase;
pub mod generic;
pub mod ledger_live;
pub mod metamask;
pub mod model;
pub mod observation;
pub mod okx;
pub mod phantom;
pub mod platform;
pub mod rabby;
pub mod safepal;
pub mod trust_wallet;
pub mod validation;
pub mod version;

pub use adapter::{built_in_adapters, find_adapter, WalletAdapter};
pub use cache::CacheState;
pub use capability::WalletApiCapability;
pub use model::WalletIdentity;
pub use observation::WalletObservation;
pub use platform::WalletPlatform;
pub use validation::validate_observation;
pub use version::WalletVersion;
