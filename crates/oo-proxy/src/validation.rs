// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-proxy/src/validation.rs
// Purpose : Validate contract addresses before they enter resolution.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Validate contract addresses before they enter resolution.

use crate::error::{ProxyError, ProxyResult};

/// Validates a `0x`-prefixed 20-byte hexadecimal address.
///
/// Checksummed capitalization (EIP-55) is accepted but not verified here: a
/// wrongly capitalized address still names the same account, and rejecting it
/// would refuse valid input over a display convention. Callers that need
/// checksum verification should add it explicitly.
pub fn validate_address(value: &str) -> ProxyResult<[u8; 20]> {
    let body = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
        .ok_or_else(|| ProxyError::InvalidAddress(value.to_owned()))?;

    if body.len() != 40 {
        return Err(ProxyError::InvalidAddress(value.to_owned()));
    }

    let mut address = [0u8; 20];
    for (index, chunk) in body.as_bytes().chunks(2).enumerate() {
        let text =
            std::str::from_utf8(chunk).map_err(|_| ProxyError::InvalidAddress(value.to_owned()))?;
        address[index] = u8::from_str_radix(text, 16)
            .map_err(|_| ProxyError::InvalidAddress(value.to_owned()))?;
    }
    Ok(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_well_formed_address_is_accepted() {
        let address = validate_address("0xdac17f958d2ee523a2206206994597c13d831ec7").unwrap();
        assert_eq!(address[0], 0xda);
        assert_eq!(address[19], 0xc7);
    }

    #[test]
    fn checksummed_capitalization_is_accepted() {
        assert!(validate_address("0xDAC17F958D2ee523a2206206994597C13D831Ec7").is_ok());
    }

    #[test]
    fn a_missing_prefix_is_rejected() {
        assert!(validate_address("dac17f958d2ee523a2206206994597c13d831ec7").is_err());
    }

    #[test]
    fn a_wrong_length_is_rejected() {
        assert!(validate_address("0xdac17f").is_err());
    }

    #[test]
    fn non_hex_characters_are_rejected() {
        assert!(validate_address(&format!("0x{}", "zz".repeat(20))).is_err());
    }
}
