pub mod artifacts;
mod constants;

pub use constants::*;

use starknet_crypto::FieldElement;

/// Convert decimal or hex string to FieldElement
pub fn str_to_felt(string: &str) -> FieldElement {
    if string.starts_with("0x") {
        FieldElement::from_hex_be(string).expect("Invalid hex string")
    } else {
        FieldElement::from_dec_str(string).expect("Invalid decimal string")
    }
}
