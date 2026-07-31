// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-config/src/environment.rs
// Purpose : Environment overrides and credential handling.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Environment overrides and credential handling.
//!
//! Overrides exist so an operator can change where output is written or how
//! strict a run is without editing a versioned file. Every override is recorded
//! by name, so a report can state that a run did not use the committed
//! configuration.
//!
//! Credentials never enter the configuration model as plain strings. They are
//! wrapped in [`Secret`], whose `Debug` and `Display` implementations redact the
//! value, so a credential cannot reach a log, a report or an error message by
//! accident.

use std::{collections::BTreeMap, fmt, path::PathBuf};

use crate::{
    error::{ConfigError, ConfigResult},
    model::{Config, LogFormat, LogLevel, RuntimeEnvironment, WdrpConfidence},
};

/// Prefix every Origin Observer environment variable shares.
pub const ENVIRONMENT_PREFIX: &str = "OO_";

/// A credential that must never be printed.
///
/// The value is available only through [`Secret::expose`], which makes every
/// use of a credential visible at the call site during review.
#[derive(Clone, PartialEq, Eq)]
pub struct Secret {
    value: String,
}

impl Secret {
    /// Wraps a credential value.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// Returns the credential. Every call site is a deliberate disclosure.
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.value
    }

    /// Returns whether the credential is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Secret(redacted)")
    }
}

impl fmt::Display for Secret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("redacted")
    }
}

/// Credentials resolved from the environment, keyed by provider identifier.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Credentials {
    entries: BTreeMap<String, Secret>,
}

impl Credentials {
    /// Returns the credential for a provider, if one was supplied.
    #[must_use]
    pub fn get(&self, provider_id: &str) -> Option<&Secret> {
        self.entries.get(provider_id)
    }

    /// Returns whether a credential is present for a provider.
    #[must_use]
    pub fn contains(&self, provider_id: &str) -> bool {
        self.entries.contains_key(provider_id)
    }

    /// Returns the provider identifiers that have credentials.
    ///
    /// Identifiers are safe to log; the values behind them are not.
    #[must_use]
    pub fn provider_ids(&self) -> Vec<&str> {
        self.entries.keys().map(String::as_str).collect()
    }

    /// Inserts a credential.
    pub fn insert(&mut self, provider_id: impl Into<String>, secret: Secret) {
        self.entries.insert(provider_id.into(), secret);
    }
}

/// Overrides read from the environment.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnvironmentOverrides {
    environment: Option<RuntimeEnvironment>,
    log_level: Option<LogLevel>,
    log_format: Option<LogFormat>,
    data_root: Option<PathBuf>,
    request_timeout_seconds: Option<u64>,
    max_retries: Option<u32>,
    allow_unpinned_latest_block: Option<bool>,
    minimum_accepted_confidence: Option<WdrpConfidence>,
    credentials: Credentials,
    applied: Vec<String>,
}

impl EnvironmentOverrides {
    /// Reads overrides from the process environment.
    pub fn from_env() -> ConfigResult<Self> {
        Self::from_pairs(std::env::vars())
    }

    /// Reads overrides from an explicit set of variables.
    ///
    /// Taking the variables as an argument keeps the parsing rules testable
    /// without mutating the process environment, which would make tests
    /// order-dependent.
    pub fn from_pairs<I, K, V>(pairs: I) -> ConfigResult<Self>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mut overrides = Self::default();
        for (key, value) in pairs {
            let key = key.as_ref();
            let value = value.as_ref();
            if !key.starts_with(ENVIRONMENT_PREFIX) {
                continue;
            }
            match key {
                "OO_ENVIRONMENT" => {
                    overrides.environment = Some(parse_environment(key, value)?);
                    overrides.applied.push(key.to_owned());
                }
                "OO_LOG_LEVEL" => {
                    overrides.log_level = Some(parse_log_level(key, value)?);
                    overrides.applied.push(key.to_owned());
                }
                "OO_LOG_FORMAT" => {
                    overrides.log_format = Some(parse_log_format(key, value)?);
                    overrides.applied.push(key.to_owned());
                }
                "OO_DATA_ROOT" => {
                    if value.trim().is_empty() {
                        return Err(ConfigError::InvalidOverride {
                            variable: key.to_owned(),
                            expected: "a directory path".to_owned(),
                            found: String::new(),
                        });
                    }
                    overrides.data_root = Some(PathBuf::from(value));
                    overrides.applied.push(key.to_owned());
                }
                "OO_RPC_TIMEOUT_SECONDS" => {
                    overrides.request_timeout_seconds = Some(parse_number(
                        key,
                        value,
                        "a positive whole number of seconds",
                    )?);
                    overrides.applied.push(key.to_owned());
                }
                "OO_RPC_MAX_RETRIES" => {
                    let retries: u64 = parse_number(key, value, "a whole number of retries")?;
                    overrides.max_retries =
                        Some(
                            u32::try_from(retries).map_err(|_| ConfigError::InvalidOverride {
                                variable: key.to_owned(),
                                expected: "a retry count that fits in 32 bits".to_owned(),
                                found: value.to_owned(),
                            })?,
                        );
                    overrides.applied.push(key.to_owned());
                }
                "OO_ALLOW_UNPINNED_LATEST_BLOCK" => {
                    overrides.allow_unpinned_latest_block = Some(parse_bool(key, value)?);
                    overrides.applied.push(key.to_owned());
                }
                "OO_MINIMUM_CONFIDENCE" => {
                    overrides.minimum_accepted_confidence =
                        Some(WdrpConfidence::parse(value).ok_or_else(|| {
                            ConfigError::InvalidOverride {
                                variable: key.to_owned(),
                                expected: "one of L0, L1, L2, L3, L4 or L5".to_owned(),
                                found: value.to_owned(),
                            }
                        })?);
                    overrides.applied.push(key.to_owned());
                }
                _ => {
                    if let Some(provider) = provider_from_api_key_variable(key) {
                        // The value is never echoed, not even on failure.
                        overrides
                            .credentials
                            .insert(provider, Secret::new(value.to_owned()));
                        overrides.applied.push(key.to_owned());
                    }
                }
            }
        }
        overrides.applied.sort();
        Ok(overrides)
    }

    /// Applies the overrides to a configuration.
    pub fn apply(&self, config: &mut Config) {
        if let Some(environment) = self.environment {
            config.application.environment = environment;
        }
        if let Some(level) = self.log_level {
            config.logging.level = level;
        }
        if let Some(format) = self.log_format {
            config.logging.format = format;
        }
        if let Some(root) = &self.data_root {
            config.data.root.clone_from(root);
        }
        if let Some(timeout) = self.request_timeout_seconds {
            config.rpc.request_timeout_seconds = timeout;
        }
        if let Some(retries) = self.max_retries {
            config.rpc.max_retries = retries;
        }
        if let Some(allow) = self.allow_unpinned_latest_block {
            config.rpc.allow_unpinned_latest_block = allow;
        }
        if let Some(confidence) = self.minimum_accepted_confidence {
            config.research.minimum_accepted_confidence = confidence;
        }
    }

    /// Returns the credentials read from the environment.
    #[must_use]
    pub const fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    /// Returns the names of the variables that were applied, in sorted order.
    ///
    /// A run that overrode configuration must be able to say so; the names are
    /// recorded, the values are not.
    #[must_use]
    pub fn applied_variables(&self) -> &[String] {
        &self.applied
    }

    /// Returns whether any override was applied.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.applied.is_empty()
    }
}

fn provider_from_api_key_variable(key: &str) -> Option<String> {
    let rest = key.strip_prefix("OO_PROVIDER_")?;
    let provider = rest.strip_suffix("_API_KEY")?;
    if provider.is_empty() {
        return None;
    }
    Some(provider.to_ascii_lowercase().replace('_', "-"))
}

fn parse_environment(key: &str, value: &str) -> ConfigResult<RuntimeEnvironment> {
    match value.trim().to_ascii_lowercase().as_str() {
        "development" => Ok(RuntimeEnvironment::Development),
        "research" => Ok(RuntimeEnvironment::Research),
        "publication" => Ok(RuntimeEnvironment::Publication),
        _ => Err(ConfigError::InvalidOverride {
            variable: key.to_owned(),
            expected: "development, research or publication".to_owned(),
            found: value.to_owned(),
        }),
    }
}

fn parse_log_level(key: &str, value: &str) -> ConfigResult<LogLevel> {
    match value.trim().to_ascii_lowercase().as_str() {
        "error" => Ok(LogLevel::Error),
        "warn" => Ok(LogLevel::Warn),
        "info" => Ok(LogLevel::Info),
        "debug" => Ok(LogLevel::Debug),
        "trace" => Ok(LogLevel::Trace),
        _ => Err(ConfigError::InvalidOverride {
            variable: key.to_owned(),
            expected: "error, warn, info, debug or trace".to_owned(),
            found: value.to_owned(),
        }),
    }
}

fn parse_log_format(key: &str, value: &str) -> ConfigResult<LogFormat> {
    match value.trim().to_ascii_lowercase().as_str() {
        "pretty" => Ok(LogFormat::Pretty),
        "json" => Ok(LogFormat::Json),
        _ => Err(ConfigError::InvalidOverride {
            variable: key.to_owned(),
            expected: "pretty or json".to_owned(),
            found: value.to_owned(),
        }),
    }
}

fn parse_bool(key: &str, value: &str) -> ConfigResult<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        _ => Err(ConfigError::InvalidOverride {
            variable: key.to_owned(),
            expected: "true or false".to_owned(),
            found: value.to_owned(),
        }),
    }
}

fn parse_number(key: &str, value: &str, expected: &str) -> ConfigResult<u64> {
    value
        .trim()
        .parse::<u64>()
        .map_err(|_| ConfigError::InvalidOverride {
            variable: key.to_owned(),
            expected: expected.to_owned(),
            found: value.to_owned(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_secret_is_redacted_in_debug_and_display_output() {
        let secret = Secret::new("super-secret-key");
        assert_eq!(format!("{secret:?}"), "Secret(redacted)");
        assert_eq!(format!("{secret}"), "redacted");
        assert!(!format!("{secret:?}").contains("super-secret-key"));
        assert_eq!(secret.expose(), "super-secret-key");
    }

    #[test]
    fn credentials_are_keyed_by_provider_identifier() {
        let overrides = EnvironmentOverrides::from_pairs([
            ("OO_PROVIDER_COIN_GECKO_API_KEY", "abc123"),
            ("PATH", "/usr/bin"),
        ])
        .expect("overrides");

        assert!(overrides.credentials().contains("coin-gecko"));
        assert_eq!(
            overrides
                .credentials()
                .get("coin-gecko")
                .map(Secret::expose),
            Some("abc123")
        );
        assert_eq!(overrides.credentials().provider_ids(), vec!["coin-gecko"]);
    }

    #[test]
    fn applied_variable_names_are_recorded_but_values_are_not() {
        let overrides = EnvironmentOverrides::from_pairs([
            ("OO_LOG_LEVEL", "debug"),
            ("OO_PROVIDER_ETHERSCAN_API_KEY", "secret-value"),
        ])
        .expect("overrides");

        let recorded = overrides.applied_variables().join(",");
        assert!(recorded.contains("OO_LOG_LEVEL"));
        assert!(recorded.contains("OO_PROVIDER_ETHERSCAN_API_KEY"));
        assert!(!recorded.contains("secret-value"));
    }

    #[test]
    fn unknown_variables_are_ignored_rather_than_rejected() {
        // The process environment belongs to the operator; an unrelated
        // variable must not fail a run.
        let overrides =
            EnvironmentOverrides::from_pairs([("OO_SOMETHING_ELSE", "value")]).expect("overrides");
        assert!(overrides.is_empty());
    }

    #[test]
    fn an_invalid_override_names_the_variable_and_the_accepted_form() {
        let error = EnvironmentOverrides::from_pairs([("OO_MINIMUM_CONFIDENCE", "L9")])
            .expect_err("must reject");
        let message = error.to_string();
        assert!(message.contains("OO_MINIMUM_CONFIDENCE"), "{message}");
        assert!(message.contains("L0"), "{message}");
    }

    #[test]
    fn numeric_overrides_reject_non_numeric_values() {
        assert!(EnvironmentOverrides::from_pairs([("OO_RPC_TIMEOUT_SECONDS", "soon")]).is_err());
        assert!(EnvironmentOverrides::from_pairs([("OO_RPC_MAX_RETRIES", "-1")]).is_err());
    }

    #[test]
    fn boolean_overrides_accept_the_common_spellings() {
        for value in ["true", "1", "yes"] {
            let overrides =
                EnvironmentOverrides::from_pairs([("OO_ALLOW_UNPINNED_LATEST_BLOCK", value)])
                    .expect("overrides");
            assert_eq!(overrides.allow_unpinned_latest_block, Some(true));
        }
        for value in ["false", "0", "no"] {
            let overrides =
                EnvironmentOverrides::from_pairs([("OO_ALLOW_UNPINNED_LATEST_BLOCK", value)])
                    .expect("overrides");
            assert_eq!(overrides.allow_unpinned_latest_block, Some(false));
        }
        assert!(
            EnvironmentOverrides::from_pairs([("OO_ALLOW_UNPINNED_LATEST_BLOCK", "maybe")])
                .is_err()
        );
    }
}
