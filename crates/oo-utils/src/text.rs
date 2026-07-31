// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-utils/src/text.rs
// Purpose : Deterministic text normalization for comparable observations.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Deterministic text normalization for comparable observations.
//!
//! Provider and registry responses describe the same asset with different
//! spacing, casing and invisible characters. Comparing those raw strings would
//! report a difference where none exists, so text is normalized before
//! comparison. Normalization never replaces the raw value: evidence records
//! keep the original, and the normalized form exists only for matching.

/// Collapses whitespace runs into single spaces and trims the ends.
///
/// Control characters are treated as whitespace, so a value carrying a stray
/// newline or tab compares equal to its clean counterpart.
#[must_use]
pub fn collapse_whitespace(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut pending_space = false;
    for character in value.chars() {
        if character.is_whitespace() || character.is_control() {
            pending_space = !result.is_empty();
            continue;
        }
        if pending_space {
            result.push(' ');
            pending_space = false;
        }
        result.push(character);
    }
    result
}

/// Removes zero-width and bidirectional control characters.
///
/// These characters are invisible in a report but change string equality, and
/// they are a known way to make two different assets look identical to a
/// reader. They are dropped from the comparison form and reported separately.
#[must_use]
pub fn strip_invisible(value: &str) -> String {
    value
        .chars()
        .filter(|character| !is_invisible(*character))
        .collect()
}

/// Returns whether a character is invisible in rendered text.
#[must_use]
pub const fn is_invisible(character: char) -> bool {
    matches!(
        character,
        '\u{00AD}'
            | '\u{200B}'..='\u{200F}'
            | '\u{202A}'..='\u{202E}'
            | '\u{2060}'..='\u{2064}'
            | '\u{FEFF}'
    )
}

/// Returns whether a value contains any invisible character.
///
/// A caller that is comparing asset names should report this rather than
/// silently normalizing it away.
#[must_use]
pub fn contains_invisible(value: &str) -> bool {
    value.chars().any(is_invisible)
}

/// Produces the canonical comparison form of a value.
///
/// Invisible characters are removed, whitespace is collapsed and ASCII letters
/// are lowercased. Non-ASCII letters keep their case, because case folding
/// outside ASCII is locale-dependent and would make the result non-deterministic.
#[must_use]
pub fn normalize(value: &str) -> String {
    collapse_whitespace(&strip_invisible(value)).to_ascii_lowercase()
}

/// Returns whether two values are equal after normalization.
#[must_use]
pub fn equivalent(left: &str, right: &str) -> bool {
    normalize(left) == normalize(right)
}

/// Produces a lowercase identifier suitable for a file name or a key.
///
/// Every run of non-alphanumeric characters collapses into a single `-`, and
/// the result never begins or ends with a separator.
#[must_use]
pub fn slug(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    let mut pending_separator = false;
    for character in strip_invisible(value).chars() {
        if character.is_ascii_alphanumeric() {
            if pending_separator && !result.is_empty() {
                result.push('-');
            }
            pending_separator = false;
            result.push(character.to_ascii_lowercase());
        } else {
            pending_separator = true;
        }
    }
    result
}

/// Truncates a value to `maximum` characters for display.
///
/// The result is never longer than `maximum`, and truncation is marked with a
/// single ellipsis so a reader can tell the value was shortened. Truncation
/// happens on character boundaries, never inside a multi-byte character.
#[must_use]
pub fn truncate(value: &str, maximum: usize) -> String {
    if value.chars().count() <= maximum {
        return value.to_owned();
    }
    if maximum == 0 {
        return String::new();
    }
    let keep = maximum - 1;
    let mut result: String = value.chars().take(keep).collect();
    result.push('…');
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_collapses_and_trims() {
        assert_eq!(collapse_whitespace("  Tether   USD \n"), "Tether USD");
        assert_eq!(collapse_whitespace("\t\n"), "");
    }

    #[test]
    fn control_characters_are_treated_as_whitespace() {
        assert_eq!(collapse_whitespace("Tether\u{0007}USD"), "Tether USD");
    }

    #[test]
    fn invisible_characters_are_detected_and_removed() {
        let deceptive = "Tether\u{200B}USD";
        assert!(contains_invisible(deceptive));
        assert_eq!(strip_invisible(deceptive), "TetherUSD");
        assert!(!contains_invisible("Tether USD"));
    }

    #[test]
    fn normalization_makes_provider_spellings_comparable() {
        assert!(equivalent("Tether USD", "  tether   usd  "));
        assert!(equivalent("USDT", "usdt"));
        assert!(!equivalent("USDT", "USDC"));
    }

    #[test]
    fn normalization_leaves_non_ascii_case_untouched() {
        // ASCII-only folding keeps the result independent of locale.
        assert_eq!(normalize("İSTANBUL"), "İstanbul".to_ascii_lowercase());
    }

    #[test]
    fn slug_produces_a_stable_identifier() {
        assert_eq!(slug("Ethereum Mainnet"), "ethereum-mainnet");
        assert_eq!(slug("  USDT (ERC-20) "), "usdt-erc-20");
        assert_eq!(slug("---"), "");
    }

    #[test]
    fn truncation_respects_character_boundaries() {
        assert_eq!(truncate("USDT", 10), "USDT");
        assert_eq!(truncate("Tether USD", 6), "Tethe…");
        assert_eq!(truncate("ıııııı", 3), "ıı…");
        assert_eq!(truncate("abc", 0), "");
    }
}
