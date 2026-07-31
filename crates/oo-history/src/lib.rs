// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-history/src/lib.rs
// Purpose : Model historical wallet-recognition case studies.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Model historical wallet-recognition case studies.

pub mod asset_case_study;
pub mod confidence;
pub mod provider_timeline;
pub mod recognition_timeline;
pub mod source;
pub mod timeline;
pub mod validation;

pub use asset_case_study::AssetCaseStudy;
pub use confidence::HistoricalClaim;
pub use provider_timeline::{ProviderEvent, ProviderTimeline};
pub use recognition_timeline::{RecognitionEvent, RecognitionTimeline};
pub use source::HistoricalSource;
pub use timeline::{Timeline, TimelineEntry};
pub use validation::{validate_case_study, HistoryValidationError};
