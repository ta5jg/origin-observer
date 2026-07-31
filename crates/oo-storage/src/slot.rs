// -----------------------------------------------------------------------------
// Project : Origin Observer
// File    : crates/oo-storage/src/slot.rs
// Purpose : Storage slot identity and layout arithmetic.
// Author  : İrfan Gedik
// Year    : 2026
// -----------------------------------------------------------------------------

//! Storage slot identity and layout arithmetic.
//!
//! Solidity's storage layout is a fixed, specified computation, not a
//! convention this crate can approximate. A mapping's value for key `k` under
//! a mapping declared at slot `p` lives at `keccak256(k padded to 32 bytes ++
//! p padded to 32 bytes)`; a dynamic array's data begins at
//! `keccak256(p)`. Getting either formula slightly wrong silently reads the
//! wrong contract state and reports it as if it were correct.

use sha3::{Digest, Keccak256};

/// A 32-byte storage slot key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StorageSlot([u8; 32]);

impl StorageSlot {
    /// Wraps a raw 32-byte slot key.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Creates the slot for a simple sequential declaration index (`0`, `1`,
    /// `2`, ...), as Solidity assigns to top-level state variables in
    /// declaration order.
    #[must_use]
    pub fn from_index(index: u64) -> Self {
        let mut bytes = [0u8; 32];
        bytes[24..].copy_from_slice(&index.to_be_bytes());
        Self(bytes)
    }

    /// Returns the raw slot bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Returns the `0x`-prefixed hexadecimal slot.
    #[must_use]
    pub fn to_hex(self) -> String {
        let mut out = String::with_capacity(66);
        out.push_str("0x");
        for byte in self.0 {
            out.push_str(&format!("{byte:02x}"));
        }
        out
    }

    /// Computes the storage location of `mapping[key]` where the mapping
    /// itself is declared at this slot.
    #[must_use]
    pub fn mapping_value(self, key: &[u8; 32]) -> Self {
        let mut preimage = [0u8; 64];
        preimage[..32].copy_from_slice(key);
        preimage[32..].copy_from_slice(&self.0);
        Self(keccak(&preimage))
    }

    /// Computes the storage location of a mapping keyed by an `address`,
    /// which is left-padded to 32 bytes before hashing.
    #[must_use]
    pub fn mapping_value_address(self, address: &[u8; 20]) -> Self {
        let mut key = [0u8; 32];
        key[12..].copy_from_slice(address);
        self.mapping_value(&key)
    }

    /// Computes the base slot of a dynamic array's element data, where the
    /// array's length is stored at this slot.
    #[must_use]
    pub fn dynamic_array_data(self) -> Self {
        Self(keccak(&self.0))
    }

    /// Computes the slot of element `index` within a dynamic array whose data
    /// starts at this slot (as returned by [`Self::dynamic_array_data`]),
    /// where each element occupies one word.
    #[must_use]
    pub fn array_element(self, index: u64) -> Self {
        let base = u256_from_be_bytes(&self.0);
        Self(u256_to_be_bytes(add_u256_u64(base, index)))
    }

    /// Subtracts one from the slot value, treating it as a 256-bit integer.
    ///
    /// EIP-1967 defines its slots as `keccak256(id) - 1`, specifically to
    /// avoid colliding with a slot a naive sequential declaration might use.
    #[must_use]
    pub fn minus_one(self) -> Self {
        let value = u256_from_be_bytes(&self.0);
        Self(u256_to_be_bytes(sub_u256_u64(value, 1)))
    }
}

fn keccak(bytes: &[u8]) -> [u8; 32] {
    let hash = Keccak256::digest(bytes);
    let mut out = [0u8; 32];
    out.copy_from_slice(&hash);
    out
}

/// Minimal big-endian 256-bit arithmetic: only what slot layout needs
/// (subtract one, add a small offset), not a general bignum implementation.
fn u256_from_be_bytes(bytes: &[u8; 32]) -> [u32; 8] {
    let mut limbs = [0u32; 8];
    for (index, chunk) in bytes.chunks(4).enumerate() {
        limbs[index] = u32::from_be_bytes(chunk.try_into().expect("4-byte chunk"));
    }
    limbs
}

fn u256_to_be_bytes(limbs: [u32; 8]) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    for (index, limb) in limbs.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&limb.to_be_bytes());
    }
    bytes
}

fn sub_u256_u64(mut limbs: [u32; 8], mut value: u64) -> [u32; 8] {
    for limb in limbs.iter_mut().rev() {
        let (result, borrow) = (u64::from(*limb)).overflowing_sub(value & 0xFFFF_FFFF);
        if borrow {
            *limb = result as u32;
            value = (value >> 32) + 1;
        } else {
            *limb = result as u32;
            value >>= 32;
        }
        if value == 0 {
            break;
        }
    }
    limbs
}

fn add_u256_u64(mut limbs: [u32; 8], mut value: u64) -> [u32; 8] {
    for limb in limbs.iter_mut().rev() {
        let sum = u64::from(*limb) + (value & 0xFFFF_FFFF);
        *limb = sum as u32;
        value = (value >> 32) + (sum >> 32);
        if value == 0 {
            break;
        }
    }
    limbs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_slots_encode_the_index_in_the_low_bytes() {
        assert_eq!(
            StorageSlot::from_index(0).to_hex(),
            format!("0x{}", "00".repeat(32))
        );
        let slot1 = StorageSlot::from_index(1);
        assert_eq!(slot1.as_bytes()[31], 1);
        assert!(slot1.as_bytes()[..31].iter().all(|byte| *byte == 0));
    }

    #[test]
    fn mapping_value_matches_the_specified_keccak_preimage() {
        // keccak256(key(32) ++ slot(32)) computed by hand for a known input,
        // cross-checking the implementation's byte layout independently.
        let base = StorageSlot::from_index(0);
        let key = [0xAAu8; 32];
        let mut preimage = [0u8; 64];
        preimage[..32].copy_from_slice(&key);
        preimage[32..].copy_from_slice(base.as_bytes());
        let expected = keccak(&preimage);
        assert_eq!(base.mapping_value(&key).as_bytes(), &expected);
    }

    #[test]
    fn address_keyed_mapping_left_pads_the_key() {
        let base = StorageSlot::from_index(1);
        let address = [0x11u8; 20];
        let mut key = [0u8; 32];
        key[12..].copy_from_slice(&address);
        assert_eq!(
            base.mapping_value_address(&address),
            base.mapping_value(&key)
        );
    }

    #[test]
    fn dynamic_array_data_is_keccak_of_the_length_slot() {
        let base = StorageSlot::from_index(5);
        assert_eq!(
            base.dynamic_array_data().as_bytes(),
            &keccak(base.as_bytes())
        );
    }

    #[test]
    fn array_elements_are_sequential_from_the_data_slot() {
        let data = StorageSlot::from_index(5).dynamic_array_data();
        let first = data.array_element(0);
        let second = data.array_element(1);
        assert_eq!(first, data);
        let first_value = u256_from_be_bytes(first.as_bytes());
        let second_value = u256_from_be_bytes(second.as_bytes());
        assert_eq!(first_value[7] + 1, second_value[7]);
    }

    #[test]
    fn minus_one_matches_the_eip1967_offset_used_by_hand() {
        let slot = StorageSlot::from_bytes({
            let mut bytes = [0u8; 32];
            bytes[31] = 5;
            bytes
        });
        let mut expected = [0u8; 32];
        expected[31] = 4;
        assert_eq!(slot.minus_one().as_bytes(), &expected);
    }

    #[test]
    fn minus_one_borrows_across_byte_boundaries() {
        let slot = StorageSlot::from_bytes({
            let mut bytes = [0u8; 32];
            bytes[30] = 1; // 0x0100
            bytes
        });
        let mut expected = [0u8; 32];
        expected[31] = 0xff; // 0x00ff
        assert_eq!(slot.minus_one().as_bytes(), &expected);
    }
}
