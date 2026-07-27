// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-model/src/confidence.rs
// Purpose : Confidence model.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Confidence model.
//!
//! Confidence is a first-class concept inside Origin Observer.
//!
//! Every discovered object, observation, evidence, report or experiment can
//! carry a confidence assessment describing how trustworthy that information
//! is and why that conclusion was reached.
//!
//! Confidence is deterministic and immutable unless explicitly recalculated.

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt;
use std::time::SystemTime;

use oo_core::error::invalid_argument;
use oo_core::{ConfidenceId, Error, Result};
use uuid::Uuid;

/// Maximum reason length.
pub const MAX_REASON_LENGTH: usize = 512;

/// Maximum source length.
pub const MAX_SOURCE_LENGTH: usize = 128;

/// Maximum notes length.
pub const MAX_NOTES_LENGTH: usize = 2048;

/// Maximum tags.
pub const MAX_TAG_COUNT: usize = 64;

/// Confidence level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum ConfidenceLevel {
    None,

    VeryLow,

    Low,

    #[default]
    Medium,

    High,

    VeryHigh,

    Certain,
}

impl ConfidenceLevel {
    #[must_use]
    pub const fn percentage(self) -> u8 {
        match self {
            Self::None => 0,
            Self::VeryLow => 10,
            Self::Low => 30,
            Self::Medium => 50,
            Self::High => 75,
            Self::VeryHigh => 90,
            Self::Certain => 100,
        }
    }
}

/// Confidence source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConfidenceSource {
    Manual,

    Evidence,

    Discovery,

    Experiment,

    Provider,

    Snapshot,

    Report,

    Consensus,

    Heuristic,

    Statistical,

    ArtificialIntelligence,

    User,

    Unknown,
}

/// Confidence status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConfidenceStatus {
    Active,

    Deprecated,

    Replaced,

    Archived,
}

/// Confidence score.
///
/// Value range:
///
/// 0.0 ..= 1.0
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConfidenceScore(f64);

impl ConfidenceScore {
    pub fn new(value: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&value) {
            return Err(invalid_argument(
                "confidence score must be between 0.0 and 1.0",
            ));
        }

        Ok(Self(value))
    }

    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }

    #[must_use]
    pub fn percentage(self) -> f64 {
        self.0 * 100.0
    }
}

impl Default for ConfidenceScore {
    fn default() -> Self {
        Self(0.5)
    }
}

impl fmt::Display for ConfidenceScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.percentage())
    }
}

/// Confidence assessment.
#[derive(Debug, Clone)]
pub struct Confidence {
    id: ConfidenceId,

    level: ConfidenceLevel,

    score: ConfidenceScore,

    source: ConfidenceSource,

    status: ConfidenceStatus,

    reason: String,

    notes: Option<String>,

    tags: BTreeSet<String>,
}

impl Confidence {
    /// Creates a new confidence assessment.
    pub fn new(
        level: ConfidenceLevel,
        score: ConfidenceScore,
        source: ConfidenceSource,
        reason: impl Into<String>,
    ) -> Result<Self> {
        let reason = normalize_reason(reason.into())?;

        Ok(Self {
            id: ConfidenceId::new(),
            level,
            score,
            source,
            status: ConfidenceStatus::Active,
            reason,
            notes: None,
            tags: BTreeSet::new(),
        })
    }

    #[must_use]
    pub const fn id(&self) -> ConfidenceId {
        self.id
    }

    #[must_use]
    pub const fn level(&self) -> ConfidenceLevel {
        self.level
    }

    pub const fn set_level(&mut self, level: ConfidenceLevel) {
        self.level = level;
    }

    #[must_use]
    pub const fn score(&self) -> ConfidenceScore {
        self.score
    }

    pub fn set_score(&mut self, score: ConfidenceScore) {
        self.score = score;
    }

    #[must_use]
    pub const fn source(&self) -> ConfidenceSource {
        self.source
    }

    pub const fn set_source(&mut self, source: ConfidenceSource) {
        self.source = source;
    }

    #[must_use]
    pub const fn status(&self) -> ConfidenceStatus {
        self.status
    }

    pub const fn set_status(&mut self, status: ConfidenceStatus) {
        self.status = status;
    }

    pub const fn archive(&mut self) {
        self.status = ConfidenceStatus::Archived;
    }

    pub const fn deprecate(&mut self) {
        self.status = ConfidenceStatus::Deprecated;
    }

    pub const fn replace(&mut self) {
        self.status = ConfidenceStatus::Replaced;
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn set_reason(&mut self, value: impl Into<String>) -> Result<()> {
        self.reason = normalize_reason(value.into())?;
        Ok(())
    }

    #[must_use]
    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

    pub fn set_notes(&mut self, value: impl Into<String>) -> Result<()> {
        let value = normalize(value.into(), MAX_NOTES_LENGTH, "notes")?;

        self.notes = Some(value);

        Ok(())
    }

    pub fn clear_notes(&mut self) {
        self.notes = None;
    }

    pub fn add_tag(&mut self, tag: impl Into<String>) -> Result<bool> {
        if self.tags.len() >= MAX_TAG_COUNT {
            return Err(invalid_argument("too many confidence tags"));
        }

        let tag = normalize(tag.into(), MAX_SOURCE_LENGTH, "tag")?;

        Ok(self.tags.insert(tag))
    }

    pub fn remove_tag(&mut self, tag: &str) -> bool {
        self.tags.remove(tag)
    }

    #[must_use]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }

    #[must_use]
    pub fn tags(&self) -> &BTreeSet<String> {
        &self.tags
    }

    #[must_use]
    pub fn is_certain(&self) -> bool {
        self.level == ConfidenceLevel::Certain
    }

    #[must_use]
    pub fn is_high(&self) -> bool {
        matches!(
            self.level,
            ConfidenceLevel::High | ConfidenceLevel::VeryHigh | ConfidenceLevel::Certain
        )
    }

    #[must_use]
    pub fn is_low(&self) -> bool {
        matches!(
            self.level,
            ConfidenceLevel::None | ConfidenceLevel::VeryLow | ConfidenceLevel::Low
        )
    }
}

fn normalize_reason(value: String) -> Result<String> {
    normalize(value, MAX_REASON_LENGTH, "reason")
}

fn normalize(value: String, max: usize, field: &str) -> Result<String> {
    let value = value.trim().to_owned();

    if value.is_empty() {
        return Err(invalid_argument(format!("{field} must not be empty")));
    }

    if value.len() > max {
        return Err(invalid_argument(format!("{field} exceeds maximum length")));
    }

    if value.chars().any(char::is_control) {
        return Err(invalid_argument(format!(
            "{field} contains control characters"
        )));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn score_validation() {
        assert!(ConfidenceScore::new(0.0).is_ok());
        assert!(ConfidenceScore::new(1.0).is_ok());

        assert!(ConfidenceScore::new(-0.1).is_err());

        assert!(ConfidenceScore::new(1.1).is_err());
    }

    #[test]
    fn create_confidence() {
        let confidence = Confidence::new(
            ConfidenceLevel::High,
            ConfidenceScore::new(0.85).unwrap(),
            ConfidenceSource::Evidence,
            "Validated by evidence",
        )
        .unwrap();

        assert_eq!(confidence.level(), ConfidenceLevel::High);

        assert!(confidence.is_high());
    }

    #[test]
    fn tags_work() {
        let mut confidence = Confidence::new(
            ConfidenceLevel::Medium,
            ConfidenceScore::default(),
            ConfidenceSource::Manual,
            "Manual review",
        )
        .unwrap();

        assert!(confidence.add_tag("manual").unwrap());

        assert!(confidence.has_tag("manual"));

        assert!(confidence.remove_tag("manual"));

        assert!(!confidence.has_tag("manual"));
    }

    #[test]
    fn archive() {
        let mut confidence = Confidence::new(
            ConfidenceLevel::Low,
            ConfidenceScore::default(),
            ConfidenceSource::Unknown,
            "Unknown source",
        )
        .unwrap();

        confidence.archive();

        assert_eq!(confidence.status(), ConfidenceStatus::Archived);
    }

    #[test]
    fn notes() {
        let mut confidence = Confidence::new(
            ConfidenceLevel::High,
            ConfidenceScore::new(0.9).unwrap(),
            ConfidenceSource::Consensus,
            "Consensus reached",
        )
        .unwrap();

        confidence
            .set_notes("Verified by five independent providers.")
            .unwrap();

        assert!(confidence.notes().is_some());

        confidence.clear_notes();

        assert!(confidence.notes().is_none());
    }
}

// ============================================================================
// Confidence aggregation
// ============================================================================

/// Strategy used while combining multiple confidence assessments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConfidenceAggregation {
    Average,
    WeightedAverage,
    Minimum,
    Maximum,
    Consensus,
}

/// Individual factor contributing to a confidence calculation.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfidenceFactor {
    name: String,
    weight: f64,
    score: ConfidenceScore,
}

impl ConfidenceFactor {
    pub fn new(name: impl Into<String>, weight: f64, score: ConfidenceScore) -> Result<Self> {
        let name = normalize(name.into(), MAX_SOURCE_LENGTH, "factor name")?;

        if !(0.0..=1.0).contains(&weight) {
            return Err(invalid_argument(
                "factor weight must be between 0.0 and 1.0",
            ));
        }

        Ok(Self {
            name,
            weight,
            score,
        })
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn weight(&self) -> f64 {
        self.weight
    }

    #[must_use]
    pub const fn score(&self) -> ConfidenceScore {
        self.score
    }
}

/// Collection of weighted confidence factors.
#[derive(Debug, Clone, Default)]
pub struct WeightedConfidence {
    factors: Vec<ConfidenceFactor>,
}

impl WeightedConfidence {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_factor(&mut self, factor: ConfidenceFactor) {
        self.factors.push(factor);
    }

    #[must_use]
    pub fn factor_count(&self) -> usize {
        self.factors.len()
    }

    #[must_use]
    pub fn factors(&self) -> &[ConfidenceFactor] {
        &self.factors
    }

    pub fn calculate(&self) -> Result<ConfidenceScore> {
        if self.factors.is_empty() {
            return ConfidenceScore::new(0.0);
        }

        let mut weighted = 0.0;
        let mut total = 0.0;

        for factor in &self.factors {
            weighted += factor.score().value() * factor.weight();

            total += factor.weight();
        }

        if total == 0.0 {
            return ConfidenceScore::new(0.0);
        }

        ConfidenceScore::new(weighted / total)
    }
}

/// Static confidence calculator.
pub struct ConfidenceCalculator;

impl ConfidenceCalculator {
    pub fn aggregate(
        scores: &[ConfidenceScore],
        strategy: ConfidenceAggregation,
    ) -> Result<ConfidenceScore> {
        if scores.is_empty() {
            return ConfidenceScore::new(0.0);
        }

        match strategy {
            ConfidenceAggregation::Average => {
                let total: f64 = scores.iter().map(|s| s.value()).sum();

                ConfidenceScore::new(total / scores.len() as f64)
            }

            ConfidenceAggregation::Minimum => {
                let value = scores.iter().map(|s| s.value()).fold(1.0, f64::min);

                ConfidenceScore::new(value)
            }

            ConfidenceAggregation::Maximum => {
                let value = scores.iter().map(|s| s.value()).fold(0.0, f64::max);

                ConfidenceScore::new(value)
            }

            ConfidenceAggregation::Consensus => {
                let average: f64 =
                    scores.iter().map(|s| s.value()).sum::<f64>() / scores.len() as f64;

                let variance: f64 = scores
                    .iter()
                    .map(|s| {
                        let d = s.value() - average;
                        d * d
                    })
                    .sum::<f64>()
                    / scores.len() as f64;

                let penalty = variance.sqrt() * 0.5;

                ConfidenceScore::new((average - penalty).max(0.0))
            }

            ConfidenceAggregation::WeightedAverage => {
                let mut weighted = WeightedConfidence::new();

                let weight = 1.0 / scores.len() as f64;

                for (index, score) in scores.iter().enumerate() {
                    weighted.add_factor(ConfidenceFactor::new(
                        format!("factor-{index}"),
                        weight,
                        *score,
                    )?);
                }

                weighted.calculate()
            }
        }
    }

    pub fn level_from_score(score: ConfidenceScore) -> ConfidenceLevel {
        let value = score.value();

        if value == 0.0 {
            ConfidenceLevel::None
        } else if value < 0.20 {
            ConfidenceLevel::VeryLow
        } else if value < 0.40 {
            ConfidenceLevel::Low
        } else if value < 0.60 {
            ConfidenceLevel::Medium
        } else if value < 0.80 {
            ConfidenceLevel::High
        } else if value < 0.99 {
            ConfidenceLevel::VeryHigh
        } else {
            ConfidenceLevel::Certain
        }
    }
}

// ============================================================================
// Confidence policy
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceDecision {
    Accept,
    Review,
    Reject,
}

#[derive(Debug, Clone)]
pub struct ConfidencePolicy {
    accept_threshold: ConfidenceScore,
    review_threshold: ConfidenceScore,
}

impl Default for ConfidencePolicy {
    fn default() -> Self {
        Self {
            accept_threshold: ConfidenceScore::new(0.80).unwrap(),
            review_threshold: ConfidenceScore::new(0.50).unwrap(),
        }
    }
}

impl ConfidencePolicy {
    pub fn evaluate(&self, score: ConfidenceScore) -> ConfidenceDecision {
        if score.value() >= self.accept_threshold.value() {
            ConfidenceDecision::Accept
        } else if score.value() >= self.review_threshold.value() {
            ConfidenceDecision::Review
        } else {
            ConfidenceDecision::Reject
        }
    }
}

#[cfg(test)]
mod aggregation_tests {

    use super::*;

    #[test]
    fn weighted_average() {
        let mut weighted = WeightedConfidence::new();

        weighted.add_factor(
            ConfidenceFactor::new("rpc", 0.5, ConfidenceScore::new(1.0).unwrap()).unwrap(),
        );

        weighted.add_factor(
            ConfidenceFactor::new("explorer", 0.5, ConfidenceScore::new(0.5).unwrap()).unwrap(),
        );

        let score = weighted.calculate().unwrap();

        assert!(score.value() > 0.70);
    }

    #[test]
    fn aggregate_average() {
        let scores = [
            ConfidenceScore::new(0.8).unwrap(),
            ConfidenceScore::new(0.6).unwrap(),
        ];

        let score =
            ConfidenceCalculator::aggregate(&scores, ConfidenceAggregation::Average).unwrap();

        assert_eq!(score.value(), 0.7,);
    }

    #[test]
    fn confidence_level_mapping() {
        let level = ConfidenceCalculator::level_from_score(ConfidenceScore::new(0.93).unwrap());

        assert_eq!(level, ConfidenceLevel::VeryHigh,);
    }

    #[test]
    fn policy_accept() {
        let policy = ConfidencePolicy::default();

        assert_eq!(
            policy.evaluate(ConfidenceScore::new(0.95).unwrap()),
            ConfidenceDecision::Accept,
        );
    }

    #[test]
    fn policy_review() {
        let policy = ConfidencePolicy::default();

        assert_eq!(
            policy.evaluate(ConfidenceScore::new(0.65).unwrap()),
            ConfidenceDecision::Review,
        );
    }

    #[test]
    fn policy_reject() {
        let policy = ConfidencePolicy::default();

        assert_eq!(
            policy.evaluate(ConfidenceScore::new(0.25).unwrap()),
            ConfidenceDecision::Reject,
        );
    }
}

// ============================================================================
// Confidence adjustment
// ============================================================================

/// Reason for modifying a confidence score after its initial calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConfidenceAdjustmentKind {
    Bonus,
    Penalty,
    ManualOverride,
    ProviderTrust,
    Consensus,
    HistoricalAccuracy,
    EvidenceQuality,
    StatisticalCorrection,
}

/// One confidence adjustment.
#[derive(Debug, Clone)]
pub struct ConfidenceAdjustment {
    kind: ConfidenceAdjustmentKind,
    delta: f64,
    reason: String,
}

impl ConfidenceAdjustment {
    pub fn new(
        kind: ConfidenceAdjustmentKind,
        delta: f64,
        reason: impl Into<String>,
    ) -> Result<Self> {
        if !(-1.0..=1.0).contains(&delta) {
            return Err(invalid_argument(
                "adjustment delta must be between -1.0 and 1.0",
            ));
        }

        Ok(Self {
            kind,
            delta,
            reason: normalize_reason(reason.into())?,
        })
    }

    #[must_use]
    pub const fn kind(&self) -> ConfidenceAdjustmentKind {
        self.kind
    }

    #[must_use]
    pub const fn delta(&self) -> f64 {
        self.delta
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

// ============================================================================
// Confidence history
// ============================================================================

#[derive(Debug, Clone)]
pub struct ConfidenceHistoryEntry {
    previous: ConfidenceScore,
    current: ConfidenceScore,
    adjustment: ConfidenceAdjustment,
}

impl ConfidenceHistoryEntry {
    #[must_use]
    pub const fn previous(&self) -> ConfidenceScore {
        self.previous
    }

    #[must_use]
    pub const fn current(&self) -> ConfidenceScore {
        self.current
    }

    #[must_use]
    pub fn adjustment(&self) -> &ConfidenceAdjustment {
        &self.adjustment
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConfidenceHistory {
    entries: Vec<ConfidenceHistoryEntry>,
}

impl ConfidenceHistory {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(
        &mut self,
        previous: ConfidenceScore,
        adjustment: ConfidenceAdjustment,
    ) -> Result<ConfidenceScore> {
        let value = (previous.value() + adjustment.delta()).clamp(0.0, 1.0);

        let current = ConfidenceScore::new(value)?;

        self.entries.push(ConfidenceHistoryEntry {
            previous,
            current,
            adjustment,
        });

        Ok(current)
    }

    #[must_use]
    pub fn entries(&self) -> &[ConfidenceHistoryEntry] {
        &self.entries
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    #[must_use]
    pub fn latest(&self) -> Option<ConfidenceScore> {
        self.entries.last().map(|e| e.current())
    }
}

// ============================================================================
// Confidence summary
// ============================================================================

#[derive(Debug, Clone)]
pub struct ConfidenceSummary {
    minimum: ConfidenceScore,
    maximum: ConfidenceScore,
    average: ConfidenceScore,
    median: ConfidenceScore,
    count: usize,
}

impl ConfidenceSummary {
    pub fn from_scores(scores: &[ConfidenceScore]) -> Result<Self> {
        if scores.is_empty() {
            return Err(invalid_argument("cannot summarize empty score collection"));
        }

        let mut values: Vec<f64> = scores.iter().map(|s| s.value()).collect();

        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let average = values.iter().sum::<f64>() / values.len() as f64;

        let median = if values.len() % 2 == 0 {
            let left = values[values.len() / 2 - 1];
            let right = values[values.len() / 2];

            (left + right) / 2.0
        } else {
            values[values.len() / 2]
        };

        Ok(Self {
            minimum: ConfidenceScore::new(*values.first().unwrap())?,
            maximum: ConfidenceScore::new(*values.last().unwrap())?,
            average: ConfidenceScore::new(average)?,
            median: ConfidenceScore::new(median)?,
            count: values.len(),
        })
    }

    #[must_use]
    pub const fn minimum(&self) -> ConfidenceScore {
        self.minimum
    }

    #[must_use]
    pub const fn maximum(&self) -> ConfidenceScore {
        self.maximum
    }

    #[must_use]
    pub const fn average(&self) -> ConfidenceScore {
        self.average
    }

    #[must_use]
    pub const fn median(&self) -> ConfidenceScore {
        self.median
    }

    #[must_use]
    pub const fn count(&self) -> usize {
        self.count
    }
}

// ============================================================================
// Additional tests
// ============================================================================

#[cfg(test)]
mod history_tests {

    use super::*;

    #[test]
    fn adjustment_applied() {
        let mut history = ConfidenceHistory::new();

        let updated = history
            .push(
                ConfidenceScore::new(0.50).unwrap(),
                ConfidenceAdjustment::new(
                    ConfidenceAdjustmentKind::Bonus,
                    0.25,
                    "Independent confirmation",
                )
                .unwrap(),
            )
            .unwrap();

        assert_eq!(updated.value(), 0.75,);

        assert_eq!(history.len(), 1,);
    }

    #[test]
    fn score_is_clamped() {
        let mut history = ConfidenceHistory::new();

        let updated = history
            .push(
                ConfidenceScore::new(0.95).unwrap(),
                ConfidenceAdjustment::new(ConfidenceAdjustmentKind::Bonus, 0.25, "Consensus")
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(updated.value(), 1.0,);
    }

    #[test]
    fn summary_statistics() {
        let scores = [
            ConfidenceScore::new(0.4).unwrap(),
            ConfidenceScore::new(0.6).unwrap(),
            ConfidenceScore::new(0.8).unwrap(),
            ConfidenceScore::new(1.0).unwrap(),
        ];

        let summary = ConfidenceSummary::from_scores(&scores).unwrap();

        assert_eq!(summary.minimum().value(), 0.4,);

        assert_eq!(summary.maximum().value(), 1.0,);

        assert_eq!(summary.count(), 4,);
    }
}

// ============================================================================
// Confidence rules
// ============================================================================

/// Rule evaluation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceRuleResult {
    Passed,
    Failed,
    Ignored,
}

/// A single confidence rule.
#[derive(Debug, Clone)]
pub struct ConfidenceRule {
    name: String,
    description: String,
    minimum_score: ConfidenceScore,
    enabled: bool,
}

impl ConfidenceRule {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        minimum_score: ConfidenceScore,
    ) -> Result<Self> {
        Ok(Self {
            name: normalize(name.into(), MAX_SOURCE_LENGTH, "rule name")?,
            description: normalize(description.into(), MAX_REASON_LENGTH, "rule description")?,
            minimum_score,
            enabled: true,
        })
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub const fn minimum_score(&self) -> ConfidenceScore {
        self.minimum_score
    }

    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    #[must_use]
    pub fn evaluate(&self, score: ConfidenceScore) -> ConfidenceRuleResult {
        if !self.enabled {
            return ConfidenceRuleResult::Ignored;
        }

        if score.value() >= self.minimum_score.value() {
            ConfidenceRuleResult::Passed
        } else {
            ConfidenceRuleResult::Failed
        }
    }
}

// ============================================================================
// Confidence explanation
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct ConfidenceExplanation {
    lines: Vec<String>,
}

impl ConfidenceExplanation {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, line: impl Into<String>) -> Result<()> {
        let line = normalize(line.into(), MAX_REASON_LENGTH, "explanation")?;

        self.lines.push(line);

        Ok(())
    }

    #[must_use]
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    #[must_use]
    pub fn text(&self) -> String {
        self.lines.join("\n")
    }
}

// ============================================================================
// Confidence statistics
// ============================================================================

#[derive(Debug, Clone)]
pub struct ConfidenceStatistics {
    accepted: usize,
    review: usize,
    rejected: usize,
    average: ConfidenceScore,
}

impl ConfidenceStatistics {
    pub fn from_scores(scores: &[ConfidenceScore], policy: &ConfidencePolicy) -> Result<Self> {
        if scores.is_empty() {
            return Ok(Self {
                accepted: 0,
                review: 0,
                rejected: 0,
                average: ConfidenceScore::new(0.0)?,
            });
        }

        let mut accepted = 0;
        let mut review = 0;
        let mut rejected = 0;

        let average = ConfidenceCalculator::aggregate(scores, ConfidenceAggregation::Average)?;

        for score in scores {
            match policy.evaluate(*score) {
                ConfidenceDecision::Accept => accepted += 1,

                ConfidenceDecision::Review => review += 1,

                ConfidenceDecision::Reject => rejected += 1,
            }
        }

        Ok(Self {
            accepted,
            review,
            rejected,
            average,
        })
    }

    #[must_use]
    pub const fn accepted(&self) -> usize {
        self.accepted
    }

    #[must_use]
    pub const fn review(&self) -> usize {
        self.review
    }

    #[must_use]
    pub const fn rejected(&self) -> usize {
        self.rejected
    }

    #[must_use]
    pub const fn average(&self) -> ConfidenceScore {
        self.average
    }
}

// ============================================================================
// Confidence engine
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct ConfidenceEngine {
    rules: Vec<ConfidenceRule>,
}

impl ConfidenceEngine {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, rule: ConfidenceRule) {
        self.rules.push(rule);
    }

    #[must_use]
    pub fn rules(&self) -> &[ConfidenceRule] {
        &self.rules
    }

    pub fn evaluate(&self, score: ConfidenceScore) -> ConfidenceDecision {
        for rule in &self.rules {
            if matches!(rule.evaluate(score), ConfidenceRuleResult::Failed) {
                return ConfidenceDecision::Reject;
            }
        }

        ConfidencePolicy::default().evaluate(score)
    }

    pub fn explain(&self, score: ConfidenceScore) -> Result<ConfidenceExplanation> {
        let mut explanation = ConfidenceExplanation::new();

        explanation.push(format!("Score: {}", score))?;

        explanation.push(format!(
            "Level: {:?}",
            ConfidenceCalculator::level_from_score(score)
        ))?;

        for rule in &self.rules {
            explanation.push(format!("{} -> {:?}", rule.name(), rule.evaluate(score)))?;
        }

        explanation.push(format!("Decision: {:?}", self.evaluate(score)))?;

        Ok(explanation)
    }
}

#[cfg(test)]
mod engine_tests {

    use super::*;

    #[test]
    fn rule_passes() {
        let rule = ConfidenceRule::new(
            "minimum",
            "Minimum confidence",
            ConfidenceScore::new(0.50).unwrap(),
        )
        .unwrap();

        assert_eq!(
            rule.evaluate(ConfidenceScore::new(0.80).unwrap()),
            ConfidenceRuleResult::Passed,
        );
    }

    #[test]
    fn rule_fails() {
        let rule = ConfidenceRule::new(
            "minimum",
            "Minimum confidence",
            ConfidenceScore::new(0.70).unwrap(),
        )
        .unwrap();

        assert_eq!(
            rule.evaluate(ConfidenceScore::new(0.40).unwrap()),
            ConfidenceRuleResult::Failed,
        );
    }

    #[test]
    fn explanation_contains_lines() {
        let mut engine = ConfidenceEngine::new();

        engine.add_rule(
            ConfidenceRule::new("minimum", "Minimum", ConfidenceScore::new(0.5).unwrap()).unwrap(),
        );

        let explanation = engine.explain(ConfidenceScore::new(0.75).unwrap()).unwrap();

        assert!(explanation.len() >= 3);
    }

    #[test]
    fn statistics() {
        let scores = [
            ConfidenceScore::new(0.9).unwrap(),
            ConfidenceScore::new(0.8).unwrap(),
            ConfidenceScore::new(0.2).unwrap(),
            ConfidenceScore::new(0.6).unwrap(),
        ];

        let stats =
            ConfidenceStatistics::from_scores(&scores, &ConfidencePolicy::default()).unwrap();

        assert_eq!(stats.accepted(), 2,);

        assert_eq!(stats.review(), 1,);

        assert_eq!(stats.rejected(), 1,);
    }

    #[test]
    fn engine_rejects_when_rule_fails() {
        let mut engine = ConfidenceEngine::new();

        engine.add_rule(
            ConfidenceRule::new("strict", "Strict", ConfidenceScore::new(0.90).unwrap()).unwrap(),
        );

        assert_eq!(
            engine.evaluate(ConfidenceScore::new(0.60).unwrap(),),
            ConfidenceDecision::Reject,
        );
    }
}

// ============================================================================
// Confidence profile
// ============================================================================

/// Represents confidence characteristics for a specific data source,
/// provider or discovery pipeline.
#[derive(Debug, Clone)]
pub struct ConfidenceProfile {
    name: String,
    description: String,
    base_score: ConfidenceScore,
    trust_multiplier: f64,
    enabled: bool,
}

impl ConfidenceProfile {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        base_score: ConfidenceScore,
    ) -> Result<Self> {
        Ok(Self {
            name: normalize(name.into(), MAX_SOURCE_LENGTH, "profile name")?,
            description: normalize(description.into(), MAX_REASON_LENGTH, "profile description")?,
            base_score,
            trust_multiplier: 1.0,
            enabled: true,
        })
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub const fn base_score(&self) -> ConfidenceScore {
        self.base_score
    }

    #[must_use]
    pub const fn trust_multiplier(&self) -> f64 {
        self.trust_multiplier
    }

    pub fn set_multiplier(&mut self, multiplier: f64) -> Result<()> {
        if !(0.0..=10.0).contains(&multiplier) {
            return Err(invalid_argument(
                "trust multiplier must be between 0.0 and 10.0",
            ));
        }

        self.trust_multiplier = multiplier;

        Ok(())
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn calculate(&self) -> Result<ConfidenceScore> {
        if !self.enabled {
            return ConfidenceScore::new(0.0);
        }

        ConfidenceScore::new((self.base_score.value() * self.trust_multiplier).clamp(0.0, 1.0))
    }
}

// ============================================================================
// Confidence matrix
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct ConfidenceMatrix {
    profiles: Vec<ConfidenceProfile>,
}

impl ConfidenceMatrix {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_profile(&mut self, profile: ConfidenceProfile) {
        self.profiles.push(profile);
    }

    #[must_use]
    pub fn profiles(&self) -> &[ConfidenceProfile] {
        &self.profiles
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }

    pub fn combined_score(&self) -> Result<ConfidenceScore> {
        if self.profiles.is_empty() {
            return ConfidenceScore::new(0.0);
        }

        let mut scores = Vec::with_capacity(self.profiles.len());

        for profile in &self.profiles {
            scores.push(profile.calculate()?);
        }

        ConfidenceCalculator::aggregate(&scores, ConfidenceAggregation::Average)
    }

    pub fn highest_profile(&self) -> Option<&ConfidenceProfile> {
        self.profiles.iter().max_by(|a, b| {
            a.calculate()
                .unwrap()
                .value()
                .partial_cmp(&b.calculate().unwrap().value())
                .unwrap()
        })
    }
}

// ============================================================================
// Confidence builder
// ============================================================================

#[derive(Debug, Clone)]
pub struct ConfidenceBuilder {
    level: ConfidenceLevel,
    score: ConfidenceScore,
    source: ConfidenceSource,
    status: ConfidenceStatus,
    reason: String,
    notes: String,
    tags: Vec<String>,
}

impl Default for ConfidenceBuilder {
    fn default() -> Self {
        Self {
            level: ConfidenceLevel::Medium,
            score: ConfidenceScore::default(),
            source: ConfidenceSource::Unknown,
            status: ConfidenceStatus::Active,
            reason: String::new(),
            notes: String::new(),
            tags: Vec::new(),
        }
    }
}

impl ConfidenceBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn level(mut self, level: ConfidenceLevel) -> Self {
        self.level = level;
        self
    }

    #[must_use]
    pub fn score(mut self, score: ConfidenceScore) -> Self {
        self.score = score;
        self
    }

    #[must_use]
    pub fn source(mut self, source: ConfidenceSource) -> Self {
        self.source = source;
        self
    }

    #[must_use]
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = reason.into();
        self
    }

    #[must_use]
    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = notes.into();
        self
    }

    #[must_use]
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn build(self) -> Result<Confidence> {
        let mut confidence = Confidence::new(self.level, self.score, self.source, self.reason)?;

        confidence.set_status(self.status);

        if !self.notes.is_empty() {
            confidence.set_notes(self.notes)?;
        }

        for tag in self.tags {
            confidence.add_tag(tag)?;
        }

        Ok(confidence)
    }
}

// ============================================================================
// Trait implementations
// ============================================================================

impl std::fmt::Display for ConfidenceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::None => "none",
                Self::VeryLow => "very-low",
                Self::Low => "low",
                Self::Medium => "medium",
                Self::High => "high",
                Self::VeryHigh => "very-high",
                Self::Certain => "certain",
            }
        )
    }
}

impl std::fmt::Display for ConfidenceDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Accept => "accept",
                Self::Review => "review",
                Self::Reject => "reject",
            }
        )
    }
}

#[cfg(test)]
mod builder_tests {

    use super::*;

    #[test]
    fn builder_creates_confidence() {
        let confidence = ConfidenceBuilder::new()
            .score(ConfidenceScore::new(0.90).unwrap())
            .level(ConfidenceLevel::VeryHigh)
            .source(ConfidenceSource::Evidence)
            .reason("Evidence validated")
            .tag("rpc")
            .tag("consensus")
            .build()
            .unwrap();

        assert_eq!(confidence.level(), ConfidenceLevel::VeryHigh,);

        assert_eq!(confidence.tags().len(), 2,);
    }

    #[test]
    fn profile_calculation() {
        let mut profile =
            ConfidenceProfile::new("RPC", "RPC Provider", ConfidenceScore::new(0.8).unwrap())
                .unwrap();

        profile.set_multiplier(1.10).unwrap();

        assert!(profile.calculate().unwrap().value() > 0.85);
    }

    #[test]
    fn matrix_average() {
        let mut matrix = ConfidenceMatrix::new();

        matrix.add_profile(
            ConfidenceProfile::new("A", "Provider A", ConfidenceScore::new(0.7).unwrap()).unwrap(),
        );

        matrix.add_profile(
            ConfidenceProfile::new("B", "Provider B", ConfidenceScore::new(0.9).unwrap()).unwrap(),
        );

        let result = matrix.combined_score().unwrap();

        assert!(result.value() > 0.79);
    }

    #[test]
    fn display_level() {
        assert_eq!(ConfidenceLevel::High.to_string(), "high",);
    }

    #[test]
    fn display_decision() {
        assert_eq!(ConfidenceDecision::Review.to_string(), "review",);
    }
}

// ============================================================================
// Confidence analytics
// ============================================================================

/// Statistical information derived from a collection of confidence scores.
#[derive(Debug, Clone)]
pub struct ConfidenceAnalytics {
    mean: ConfidenceScore,
    median: ConfidenceScore,
    minimum: ConfidenceScore,
    maximum: ConfidenceScore,
    variance: f64,
    standard_deviation: f64,
}

impl ConfidenceAnalytics {
    pub fn calculate(scores: &[ConfidenceScore]) -> Result<Self> {
        if scores.is_empty() {
            return Err(invalid_argument(
                "confidence analytics requires at least one score",
            ));
        }

        let mut values: Vec<f64> = scores.iter().map(|s| s.value()).collect();

        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mean = values.iter().sum::<f64>() / values.len() as f64;

        let variance = values
            .iter()
            .map(|v| {
                let diff = *v - mean;
                diff * diff
            })
            .sum::<f64>()
            / values.len() as f64;

        let standard_deviation = variance.sqrt();

        let median = if values.len() % 2 == 0 {
            let left = values[values.len() / 2 - 1];
            let right = values[values.len() / 2];

            (left + right) / 2.0
        } else {
            values[values.len() / 2]
        };

        Ok(Self {
            mean: ConfidenceScore::new(mean)?,
            median: ConfidenceScore::new(median)?,
            minimum: ConfidenceScore::new(*values.first().unwrap())?,
            maximum: ConfidenceScore::new(*values.last().unwrap())?,
            variance,
            standard_deviation,
        })
    }

    #[must_use]
    pub const fn mean(&self) -> ConfidenceScore {
        self.mean
    }

    #[must_use]
    pub const fn median(&self) -> ConfidenceScore {
        self.median
    }

    #[must_use]
    pub const fn minimum(&self) -> ConfidenceScore {
        self.minimum
    }

    #[must_use]
    pub const fn maximum(&self) -> ConfidenceScore {
        self.maximum
    }

    #[must_use]
    pub const fn variance(&self) -> f64 {
        self.variance
    }

    #[must_use]
    pub const fn standard_deviation(&self) -> f64 {
        self.standard_deviation
    }
}

// ============================================================================
// Confidence comparison
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceOrdering {
    Lower,
    Equal,
    Higher,
}

impl ConfidenceScore {
    #[must_use]
    pub fn compare(self, other: Self) -> ConfidenceOrdering {
        const EPSILON: f64 = 0.000_001;

        let diff = self.value() - other.value();

        if diff.abs() < EPSILON {
            ConfidenceOrdering::Equal
        } else if diff < 0.0 {
            ConfidenceOrdering::Lower
        } else {
            ConfidenceOrdering::Higher
        }
    }

    #[must_use]
    pub fn distance(self, other: Self) -> f64 {
        (self.value() - other.value()).abs()
    }

    #[must_use]
    pub fn stronger_than(self, other: Self) -> bool {
        self.value() > other.value()
    }

    #[must_use]
    pub fn weaker_than(self, other: Self) -> bool {
        self.value() < other.value()
    }
}

// ============================================================================
// Confidence normalization
// ============================================================================

pub struct ConfidenceNormalizer;

impl ConfidenceNormalizer {
    pub fn normalize_scores(scores: &[ConfidenceScore]) -> Result<Vec<ConfidenceScore>> {
        if scores.is_empty() {
            return Ok(Vec::new());
        }

        let min = scores
            .iter()
            .map(|s| s.value())
            .fold(f64::INFINITY, f64::min);

        let max = scores
            .iter()
            .map(|s| s.value())
            .fold(f64::NEG_INFINITY, f64::max);

        if (max - min).abs() < f64::EPSILON {
            return Ok(scores.to_vec());
        }

        scores
            .iter()
            .map(|score| ConfidenceScore::new((score.value() - min) / (max - min)))
            .collect()
    }
}

// ============================================================================
// Additional helper methods
// ============================================================================

impl Confidence {
    #[must_use]
    pub fn percentage(&self) -> f64 {
        self.score().percentage()
    }

    #[must_use]
    pub fn is_reliable(&self) -> bool {
        self.score().value() >= 0.80
    }

    #[must_use]
    pub fn requires_review(&self) -> bool {
        matches!(
            ConfidencePolicy::default().evaluate(self.score()),
            ConfidenceDecision::Review
        )
    }

    #[must_use]
    pub fn should_reject(&self) -> bool {
        matches!(
            ConfidencePolicy::default().evaluate(self.score()),
            ConfidenceDecision::Reject
        )
    }
}

// ============================================================================
// More tests
// ============================================================================

#[cfg(test)]
mod analytics_tests {

    use super::*;

    #[test]
    fn analytics_calculation() {
        let scores = [
            ConfidenceScore::new(0.40).unwrap(),
            ConfidenceScore::new(0.60).unwrap(),
            ConfidenceScore::new(0.80).unwrap(),
            ConfidenceScore::new(1.00).unwrap(),
        ];

        let analytics = ConfidenceAnalytics::calculate(&scores).unwrap();

        assert!(analytics.standard_deviation() > 0.0);

        assert_eq!(analytics.minimum().value(), 0.4,);

        assert_eq!(analytics.maximum().value(), 1.0,);
    }

    #[test]
    fn normalization() {
        let scores = [
            ConfidenceScore::new(0.20).unwrap(),
            ConfidenceScore::new(0.50).unwrap(),
            ConfidenceScore::new(0.80).unwrap(),
        ];

        let normalized = ConfidenceNormalizer::normalize_scores(&scores).unwrap();

        assert_eq!(normalized.first().unwrap().value(), 0.0,);

        assert_eq!(normalized.last().unwrap().value(), 1.0,);
    }

    #[test]
    fn compare_scores() {
        let a = ConfidenceScore::new(0.9).unwrap();

        let b = ConfidenceScore::new(0.4).unwrap();

        assert_eq!(a.compare(b), ConfidenceOrdering::Higher,);
    }

    #[test]
    fn helper_methods() {
        let confidence = ConfidenceBuilder::new()
            .score(ConfidenceScore::new(0.92).unwrap())
            .reason("verified")
            .build()
            .unwrap();

        assert!(confidence.is_reliable());

        assert_eq!(confidence.percentage(), 92.0,);
    }
}

// ============================================================================
// Advanced aggregation strategies
// ============================================================================

impl ConfidenceCalculator {
    /// Geometric mean.
    pub fn geometric_mean(scores: &[ConfidenceScore]) -> Result<ConfidenceScore> {
        if scores.is_empty() {
            return ConfidenceScore::new(0.0);
        }

        let mut product = 1.0;

        for score in scores {
            let value = score.value();

            if value <= 0.0 {
                return ConfidenceScore::new(0.0);
            }

            product *= value;
        }

        ConfidenceScore::new(product.powf(1.0 / scores.len() as f64))
    }

    /// Harmonic mean.
    pub fn harmonic_mean(scores: &[ConfidenceScore]) -> Result<ConfidenceScore> {
        if scores.is_empty() {
            return ConfidenceScore::new(0.0);
        }

        let mut denominator = 0.0;

        for score in scores {
            let value = score.value();

            if value == 0.0 {
                return ConfidenceScore::new(0.0);
            }

            denominator += 1.0 / value;
        }

        ConfidenceScore::new(scores.len() as f64 / denominator)
    }

    /// Root mean square.
    pub fn rms(scores: &[ConfidenceScore]) -> Result<ConfidenceScore> {
        if scores.is_empty() {
            return ConfidenceScore::new(0.0);
        }

        let value = scores.iter().map(|s| s.value() * s.value()).sum::<f64>() / scores.len() as f64;

        ConfidenceScore::new(value.sqrt())
    }

    /// Trimmed mean.
    pub fn trimmed_mean(scores: &[ConfidenceScore], trim_ratio: f64) -> Result<ConfidenceScore> {
        if scores.is_empty() {
            return ConfidenceScore::new(0.0);
        }

        if !(0.0..0.5).contains(&trim_ratio) {
            return Err(invalid_argument("trim ratio must be between 0.0 and 0.5"));
        }

        let mut values: Vec<f64> = scores.iter().map(|s| s.value()).collect();

        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let trim = ((values.len() as f64) * trim_ratio).floor() as usize;

        let slice = &values[trim..values.len() - trim];

        let average = slice.iter().sum::<f64>() / slice.len() as f64;

        ConfidenceScore::new(average)
    }
}

// ============================================================================
// ConfidenceScore conversions
// ============================================================================

impl TryFrom<f64> for ConfidenceScore {
    type Error = Error;

    fn try_from(value: f64) -> Result<Self> {
        Self::new(value)
    }
}

impl From<ConfidenceScore> for f64 {
    fn from(score: ConfidenceScore) -> Self {
        score.value()
    }
}

// ============================================================================
// Confidence collection
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct ConfidenceCollection {
    values: Vec<Confidence>,
}

impl ConfidenceCollection {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, confidence: Confidence) {
        self.values.push(confidence);
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[must_use]
    pub fn values(&self) -> &[Confidence] {
        &self.values
    }

    pub fn average_score(&self) -> Result<ConfidenceScore> {
        let scores: Vec<_> = self.values.iter().map(|c| c.score()).collect();

        ConfidenceCalculator::aggregate(&scores, ConfidenceAggregation::Average)
    }

    pub fn strongest(&self) -> Option<&Confidence> {
        self.values
            .iter()
            .max_by(|a, b| a.score().partial_cmp(&b.score()).unwrap())
    }

    pub fn weakest(&self) -> Option<&Confidence> {
        self.values
            .iter()
            .min_by(|a, b| a.score().partial_cmp(&b.score()).unwrap())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod advanced_tests {

    use super::*;

    #[test]
    fn geometric_mean() {
        let scores = [
            ConfidenceScore::new(0.8).unwrap(),
            ConfidenceScore::new(0.8).unwrap(),
        ];

        let score = ConfidenceCalculator::geometric_mean(&scores).unwrap();

        assert!((score.value() - 0.8).abs() < 0.0001);
    }

    #[test]
    fn harmonic_mean() {
        let scores = [
            ConfidenceScore::new(0.5).unwrap(),
            ConfidenceScore::new(1.0).unwrap(),
        ];

        let score = ConfidenceCalculator::harmonic_mean(&scores).unwrap();

        assert!(score.value() < 1.0);
    }

    #[test]
    fn rms_score() {
        let scores = [
            ConfidenceScore::new(0.6).unwrap(),
            ConfidenceScore::new(0.8).unwrap(),
        ];

        assert!(ConfidenceCalculator::rms(&scores,).unwrap().value() > 0.6);
    }

    #[test]
    fn collection_average() {
        let mut collection = ConfidenceCollection::new();

        collection.push(
            ConfidenceBuilder::new()
                .score(ConfidenceScore::new(0.8).unwrap())
                .reason("A")
                .build()
                .unwrap(),
        );

        collection.push(
            ConfidenceBuilder::new()
                .score(ConfidenceScore::new(0.6).unwrap())
                .reason("B")
                .build()
                .unwrap(),
        );

        let score = collection.average_score().unwrap();

        assert_eq!(score.value(), 0.7,);
    }

    #[test]
    fn conversion() {
        let score = ConfidenceScore::try_from(0.75).unwrap();

        let value: f64 = score.into();

        assert_eq!(value, 0.75,);
    }
}

// ============================================================================
// Confidence provenance
// ============================================================================

/// Origin of a confidence contribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConfidenceEvidenceKind {
    Rpc,
    Explorer,
    Bytecode,
    Abi,
    Event,
    Storage,
    Trace,
    Log,
    Signature,
    Metadata,
    SourceCode,
    Experiment,
    Discovery,
    Manual,
    External,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ConfidenceContribution {
    id: ConfidenceId,
    kind: ConfidenceEvidenceKind,
    source: String,
    score: ConfidenceScore,
    weight: f64,
    description: String,
}

impl ConfidenceContribution {
    pub fn new(
        kind: ConfidenceEvidenceKind,
        source: impl Into<String>,
        score: ConfidenceScore,
        weight: f64,
        description: impl Into<String>,
    ) -> Result<Self> {
        if !(0.0..=1.0).contains(&weight) {
            return Err(invalid_argument(
                "contribution weight must be between 0.0 and 1.0",
            ));
        }

        Ok(Self {
            id: ConfidenceId::new(),
            kind,
            source: normalize(source.into(), MAX_SOURCE_LENGTH, "contribution source")?,
            score,
            weight,
            description: normalize(
                description.into(),
                MAX_REASON_LENGTH,
                "contribution description",
            )?,
        })
    }

    #[must_use]
    pub const fn id(&self) -> ConfidenceId {
        self.id
    }

    #[must_use]
    pub const fn kind(&self) -> ConfidenceEvidenceKind {
        self.kind
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }

    #[must_use]
    pub const fn score(&self) -> ConfidenceScore {
        self.score
    }

    #[must_use]
    pub const fn weight(&self) -> f64 {
        self.weight
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn weighted_score(&self) -> f64 {
        self.score.value() * self.weight
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConfidenceProvenance {
    contributions: Vec<ConfidenceContribution>,
}

impl ConfidenceProvenance {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, contribution: ConfidenceContribution) {
        self.contributions.push(contribution);
    }

    #[must_use]
    pub fn contributions(&self) -> &[ConfidenceContribution] {
        &self.contributions
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.contributions.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contributions.is_empty()
    }

    pub fn calculate(&self) -> Result<ConfidenceScore> {
        if self.contributions.is_empty() {
            return ConfidenceScore::new(0.0);
        }

        let mut weighted = 0.0;
        let mut total = 0.0;

        for contribution in &self.contributions {
            weighted += contribution.weighted_score();
            total += contribution.weight();
        }

        if total == 0.0 {
            return ConfidenceScore::new(0.0);
        }

        ConfidenceScore::new(weighted / total)
    }

    #[must_use]
    pub fn strongest(&self) -> Option<&ConfidenceContribution> {
        self.contributions
            .iter()
            .max_by(|a, b| a.weighted_score().partial_cmp(&b.weighted_score()).unwrap())
    }

    #[must_use]
    pub fn weakest(&self) -> Option<&ConfidenceContribution> {
        self.contributions
            .iter()
            .min_by(|a, b| a.weighted_score().partial_cmp(&b.weighted_score()).unwrap())
    }

    #[must_use]
    pub fn by_kind(&self, kind: ConfidenceEvidenceKind) -> Vec<&ConfidenceContribution> {
        self.contributions
            .iter()
            .filter(|c| c.kind() == kind)
            .collect()
    }
}

// ============================================================================
// Confidence timeline
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceChangeKind {
    Initial,
    Increase,
    Decrease,
    Recalculated,
    ManualOverride,
    Invalidated,
}

#[derive(Debug, Clone)]
pub struct ConfidenceSnapshot {
    id: ConfidenceId,
    score: ConfidenceScore,
    level: ConfidenceLevel,
    source: ConfidenceSource,
    status: ConfidenceStatus,
    change: ConfidenceChangeKind,
    timestamp: SystemTime,
    reason: String,
}

impl ConfidenceSnapshot {
    pub fn new(
        confidence: &Confidence,
        change: ConfidenceChangeKind,
        reason: impl Into<String>,
    ) -> Result<Self> {
        Ok(Self {
            id: confidence.id(),
            score: confidence.score(),
            level: confidence.level(),
            source: confidence.source(),
            status: confidence.status(),
            change,
            timestamp: SystemTime::now(),
            reason: normalize_reason(reason.into())?,
        })
    }

    #[must_use]
    pub const fn id(&self) -> ConfidenceId {
        self.id
    }

    #[must_use]
    pub const fn score(&self) -> ConfidenceScore {
        self.score
    }

    #[must_use]
    pub const fn level(&self) -> ConfidenceLevel {
        self.level
    }

    #[must_use]
    pub const fn source(&self) -> ConfidenceSource {
        self.source
    }

    #[must_use]
    pub const fn status(&self) -> ConfidenceStatus {
        self.status
    }

    #[must_use]
    pub const fn change(&self) -> ConfidenceChangeKind {
        self.change
    }

    #[must_use]
    pub fn timestamp(&self) -> SystemTime {
        self.timestamp
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConfidenceTimeline {
    snapshots: Vec<ConfidenceSnapshot>,
}

impl ConfidenceTimeline {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, snapshot: ConfidenceSnapshot) {
        self.snapshots.push(snapshot);
    }

    #[must_use]
    pub fn snapshots(&self) -> &[ConfidenceSnapshot] {
        &self.snapshots
    }

    #[must_use]
    pub fn latest(&self) -> Option<&ConfidenceSnapshot> {
        self.snapshots.last()
    }

    #[must_use]
    pub fn first(&self) -> Option<&ConfidenceSnapshot> {
        self.snapshots.first()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }

    pub fn score_delta(&self) -> Option<f64> {
        let first = self.first()?;
        let last = self.latest()?;

        Some(last.score().value() - first.score().value())
    }

    pub fn highest(&self) -> Option<&ConfidenceSnapshot> {
        self.snapshots
            .iter()
            .max_by(|a, b| a.score().partial_cmp(&b.score()).unwrap())
    }

    pub fn lowest(&self) -> Option<&ConfidenceSnapshot> {
        self.snapshots
            .iter()
            .min_by(|a, b| a.score().partial_cmp(&b.score()).unwrap())
    }
}

// ============================================================================
// Confidence graph
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfidenceNodeId(Uuid);

impl ConfidenceNodeId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ConfidenceNodeId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ConfidenceNode {
    id: ConfidenceNodeId,
    name: String,
    confidence: Confidence,
}

impl ConfidenceNode {
    pub fn new(name: impl Into<String>, confidence: Confidence) -> Result<Self> {
        Ok(Self {
            id: ConfidenceNodeId::new(),
            name: normalize(name.into(), MAX_SOURCE_LENGTH, "node name")?,
            confidence,
        })
    }

    #[must_use]
    pub const fn id(&self) -> ConfidenceNodeId {
        self.id
    }

    #[must_use]
    pub fn confidence(&self) -> &Confidence {
        &self.confidence
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceDependencyKind {
    Supports,
    DependsOn,
    DerivedFrom,
    Validates,
    Confirms,
    Contradicts,
}

#[derive(Debug, Clone, Default)]
pub struct ConfidenceGraph {
    nodes: HashMap<ConfidenceNodeId, ConfidenceNode>,
    edges: Vec<ConfidenceEdge>,
}

impl ConfidenceGraph {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: ConfidenceNode) -> Option<ConfidenceNode> {
        self.nodes.insert(node.id(), node)
    }

    pub fn contains_node(&self, id: ConfidenceNodeId) -> bool {
        self.nodes.contains_key(&id)
    }

    pub fn node(&self, id: ConfidenceNodeId) -> Option<&ConfidenceNode> {
        self.nodes.get(&id)
    }

    pub fn node_mut(&mut self, id: ConfidenceNodeId) -> Option<&mut ConfidenceNode> {
        self.nodes.get_mut(&id)
    }

    pub fn remove_node(&mut self, id: ConfidenceNodeId) -> Option<ConfidenceNode> {
        self.edges.retain(|edge| edge.from != id && edge.to != id);

        self.nodes.remove(&id)
    }

    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }
}

impl ConfidenceGraph {
    pub fn add_edge(
        &mut self,
        from: ConfidenceNodeId,
        to: ConfidenceNodeId,
        kind: ConfidenceDependencyKind,
        weight: f64,
    ) -> Result<()> {
        if !self.nodes.contains_key(&from) {
            return Err(invalid_argument("source node not found"));
        }

        if !self.nodes.contains_key(&to) {
            return Err(invalid_argument("destination node not found"));
        }

        if !(0.0..=1.0).contains(&weight) {
            return Err(invalid_argument("edge weight must be between 0.0 and 1.0"));
        }

        self.edges.push(ConfidenceEdge {
            from,
            to,
            kind,
            weight,
            propagation: ConfidencePropagation::Preserve,
        });

        Ok(())
    }

    #[must_use]
    pub fn edges(&self) -> &[ConfidenceEdge] {
        &self.edges
    }

    #[must_use]
    pub fn outgoing(&self, node: ConfidenceNodeId) -> Vec<&ConfidenceEdge> {
        self.edges.iter().filter(|edge| edge.from == node).collect()
    }

    #[must_use]
    pub fn incoming(&self, node: ConfidenceNodeId) -> Vec<&ConfidenceEdge> {
        self.edges.iter().filter(|edge| edge.to == node).collect()
    }
}

// ============================================================================
// Graph traversal
// ============================================================================

impl ConfidenceGraph {
    #[must_use]
    pub fn children(&self, node: ConfidenceNodeId) -> Vec<&ConfidenceNode> {
        self.edges
            .iter()
            .filter(|edge| edge.from == node)
            .filter_map(|edge| self.nodes.get(&edge.to))
            .collect()
    }

    #[must_use]
    pub fn parents(&self, node: ConfidenceNodeId) -> Vec<&ConfidenceNode> {
        self.edges
            .iter()
            .filter(|edge| edge.to == node)
            .filter_map(|edge| self.nodes.get(&edge.from))
            .collect()
    }

    #[must_use]
    pub fn roots(&self) -> Vec<&ConfidenceNode> {
        self.nodes
            .values()
            .filter(|node| self.incoming(node.id()).is_empty())
            .collect()
    }

    #[must_use]
    pub fn leaves(&self) -> Vec<&ConfidenceNode> {
        self.nodes
            .values()
            .filter(|node| self.outgoing(node.id()).is_empty())
            .collect()
    }

    #[must_use]
    pub fn child_count(&self, node: ConfidenceNodeId) -> usize {
        self.outgoing(node).len()
    }

    #[must_use]
    pub fn parent_count(&self, node: ConfidenceNodeId) -> usize {
        self.incoming(node).len()
    }

    #[must_use]
    pub fn has_children(&self, node: ConfidenceNodeId) -> bool {
        !self.outgoing(node).is_empty()
    }

    #[must_use]
    pub fn has_parents(&self, node: ConfidenceNodeId) -> bool {
        !self.incoming(node).is_empty()
    }
}

impl ConfidenceGraph {
    fn descendants_impl(&self, node: ConfidenceNodeId, visited: &mut HashSet<ConfidenceNodeId>) {
        for edge in self.outgoing(node) {
            if visited.insert(edge.to) {
                self.descendants_impl(edge.to, visited);
            }
        }
    }

    #[must_use]
    pub fn descendants(&self, node: ConfidenceNodeId) -> Vec<&ConfidenceNode> {
        let mut visited = HashSet::new();

        self.descendants_impl(node, &mut visited);

        visited.iter().filter_map(|id| self.nodes.get(id)).collect()
    }

    fn ancestors_impl(&self, node: ConfidenceNodeId, visited: &mut HashSet<ConfidenceNodeId>) {
        for edge in self.incoming(node) {
            if visited.insert(edge.from) {
                self.ancestors_impl(edge.from, visited);
            }
        }
    }

    #[must_use]
    pub fn ancestors(&self, node: ConfidenceNodeId) -> Vec<&ConfidenceNode> {
        let mut visited = HashSet::new();

        self.ancestors_impl(node, &mut visited);

        visited.iter().filter_map(|id| self.nodes.get(id)).collect()
    }

    #[must_use]
    pub fn reachable(&self, from: ConfidenceNodeId, to: ConfidenceNodeId) -> bool {
        self.descendants(from).iter().any(|node| node.id() == to)
    }
}

// ============================================================================
// Graph validation
// ============================================================================

impl ConfidenceGraph {
    #[must_use]
    pub fn validate(&self) -> Result<()> {
        for edge in &self.edges {
            if !self.nodes.contains_key(&edge.from) {
                return Err(invalid_argument(
                    "graph contains edge with missing source node",
                ));
            }

            if !self.nodes.contains_key(&edge.to) {
                return Err(invalid_argument(
                    "graph contains edge with missing destination node",
                ));
            }

            if !(0.0..=1.0).contains(&edge.weight) {
                return Err(invalid_argument("graph contains invalid edge weight"));
            }
        }

        if self.detect_cycle() {
            return Err(invalid_argument("confidence graph contains a cycle"));
        }

        Ok(())
    }

    #[must_use]
    pub fn detect_cycle(&self) -> bool {
        let mut visited = HashSet::<ConfidenceNodeId>::new();

        let mut recursion = HashSet::<ConfidenceNodeId>::new();

        for id in self.nodes.keys().copied() {
            if self.detect_cycle_impl(id, &mut visited, &mut recursion) {
                return true;
            }
        }

        false
    }

    fn detect_cycle_impl(
        &self,
        node: ConfidenceNodeId,
        visited: &mut HashSet<ConfidenceNodeId>,
        recursion: &mut HashSet<ConfidenceNodeId>,
    ) -> bool {
        if recursion.contains(&node) {
            return true;
        }

        if !visited.insert(node) {
            return false;
        }

        recursion.insert(node);

        for edge in self.outgoing(node) {
            if self.detect_cycle_impl(edge.to, visited, recursion) {
                return true;
            }
        }

        recursion.remove(&node);

        false
    }
}

// ============================================================================
// Topological ordering
// ============================================================================

impl ConfidenceGraph {
    pub fn topological_sort(&self) -> Result<Vec<ConfidenceNodeId>> {
        self.validate()?;

        let mut indegree = HashMap::<ConfidenceNodeId, usize>::new();

        for id in self.nodes.keys().copied() {
            indegree.insert(id, 0);
        }

        for edge in &self.edges {
            *indegree.entry(edge.to).or_default() += 1;
        }

        let mut queue = VecDeque::new();

        for (id, degree) in &indegree {
            if *degree == 0 {
                queue.push_back(*id);
            }
        }

        let mut result = Vec::new();

        while let Some(node) = queue.pop_front() {
            result.push(node);

            for edge in self.outgoing(node) {
                let degree = indegree.get_mut(&edge.to).unwrap();

                *degree -= 1;

                if *degree == 0 {
                    queue.push_back(edge.to);
                }
            }
        }

        if result.len() != self.nodes.len() {
            return Err(invalid_argument("topological sort failed"));
        }

        Ok(result)
    }
}

// ============================================================================
// Confidence propagation
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConfidencePropagation {
    /// Child confidence is forwarded unchanged.
    Preserve,

    /// Child confidence is multiplied by edge weight.
    Multiply,

    /// Parent confidence becomes the minimum value.
    Minimum,

    /// Parent confidence becomes the maximum value.
    Maximum,

    /// Average of all incoming confidences.
    Average,

    /// Weighted average.
    WeightedAverage,

    /// Parent is overridden by this child.
    Override,

    /// Child confidence is ignored.
    Ignore,
}

#[derive(Debug, Clone)]
pub struct ConfidenceEdge {
    from: ConfidenceNodeId,
    to: ConfidenceNodeId,
    kind: ConfidenceDependencyKind,
    weight: f64,
    propagation: ConfidencePropagation,
}

impl ConfidenceEdge {
    pub fn new(
        from: ConfidenceNodeId,
        to: ConfidenceNodeId,
        kind: ConfidenceDependencyKind,
        weight: f64,
        propagation: ConfidencePropagation,
    ) -> Result<Self> {
        if !(0.0..=1.0).contains(&weight) {
            return Err(invalid_argument("edge weight must be between 0.0 and 1.0"));
        }

        Ok(Self {
            from,
            to,
            kind,
            weight,
            propagation,
        })
    }

    #[must_use]
    pub const fn from(&self) -> ConfidenceNodeId {
        self.from
    }

    #[must_use]
    pub const fn to(&self) -> ConfidenceNodeId {
        self.to
    }

    #[must_use]
    pub const fn kind(&self) -> ConfidenceDependencyKind {
        self.kind
    }

    #[must_use]
    pub const fn weight(&self) -> f64 {
        self.weight
    }

    #[must_use]
    pub const fn propagation(&self) -> ConfidencePropagation {
        self.propagation
    }
}

impl ConfidenceGraph {
    pub fn add_edge_with_propagation(
        &mut self,
        from: ConfidenceNodeId,
        to: ConfidenceNodeId,
        kind: ConfidenceDependencyKind,
        weight: f64,
        propagation: ConfidencePropagation,
    ) -> Result<()> {
        if !self.nodes.contains_key(&from) {
            return Err(invalid_argument("source node not found"));
        }

        if !self.nodes.contains_key(&to) {
            return Err(invalid_argument("destination node not found"));
        }

        self.edges
            .push(ConfidenceEdge::new(from, to, kind, weight, propagation)?);

        Ok(())
    }
}
