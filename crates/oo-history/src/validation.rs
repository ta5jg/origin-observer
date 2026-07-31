// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-history/src/validation.rs
// Purpose : Validate that a case study is complete and internally ordered.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Validate that a case study is complete and internally ordered.

use crate::asset_case_study::AssetCaseStudy;

/// One way a case study can fail validation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HistoryValidationError {
    /// The case study has no research question identifier.
    #[error("case study is missing a research question id")]
    MissingQuestionId,
    /// The case study has no narrative.
    #[error("case study is missing a narrative")]
    MissingNarrative,
    /// The recognition timeline is not ordered by non-decreasing timestamp.
    #[error("recognition timeline is not chronological")]
    RecognitionTimelineNotChronological,
    /// The provider timeline is not ordered by non-decreasing timestamp.
    #[error("provider timeline is not chronological")]
    ProviderTimelineNotChronological,
}

/// Validates that a case study is documented and its timelines are
/// chronological.
///
/// # Errors
///
/// Returns the first [`HistoryValidationError`] found.
pub fn validate_case_study(case_study: &AssetCaseStudy) -> Result<(), HistoryValidationError> {
    if case_study.question_id().trim().is_empty() {
        return Err(HistoryValidationError::MissingQuestionId);
    }
    if case_study.narrative().trim().is_empty() {
        return Err(HistoryValidationError::MissingNarrative);
    }
    if !case_study.recognition_timeline().is_chronological() {
        return Err(HistoryValidationError::RecognitionTimelineNotChronological);
    }
    if !case_study.provider_timeline().is_chronological() {
        return Err(HistoryValidationError::ProviderTimelineNotChronological);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use oo_core::WalletId;

    use super::*;
    use crate::recognition_timeline::RecognitionEvent;
    use crate::source::HistoricalSource;
    use crate::timeline::TimelineEntry;

    fn at(seconds: i64) -> chrono::DateTime<Utc> {
        Utc.timestamp_opt(seconds, 0).unwrap()
    }

    fn documented_case_study() -> AssetCaseStudy {
        let mut case_study = AssetCaseStudy::new("RQ-0006");
        case_study.set_narrative("USDT recognition spread across wallets between 2021 and 2022.");
        case_study
    }

    #[test]
    fn a_documented_case_study_with_ordered_timelines_is_valid() {
        assert!(validate_case_study(&documented_case_study()).is_ok());
    }

    #[test]
    fn a_case_study_without_a_narrative_is_rejected() {
        let case_study = AssetCaseStudy::new("RQ-0006");
        assert_eq!(
            validate_case_study(&case_study),
            Err(HistoryValidationError::MissingNarrative)
        );
    }

    #[test]
    fn an_out_of_order_recognition_timeline_is_rejected() {
        let mut case_study = documented_case_study();
        case_study
            .recognition_timeline_mut()
            .push(TimelineEntry::new(
                at(2),
                RecognitionEvent::new(WalletId::new(), true, HistoricalSource::new("s")),
            ));
        case_study
            .recognition_timeline_mut()
            .push(TimelineEntry::new(
                at(1),
                RecognitionEvent::new(WalletId::new(), false, HistoricalSource::new("s")),
            ));
        assert_eq!(
            validate_case_study(&case_study),
            Err(HistoryValidationError::RecognitionTimelineNotChronological)
        );
    }
}
