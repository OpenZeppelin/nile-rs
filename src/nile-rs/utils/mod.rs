pub mod constants;
pub mod fs;

use anyhow::{Context, Ok, Result};
use constants::{ADDR_BOUND, PREFIX_CONTRACT_ADDRESS};
use starknet_core::{crypto::compute_hash_on_elements, types::FieldElement};

pub fn compute_contract_address(
    salt: FieldElement,
    class_hash: FieldElement,
    constructor_calldata: &[FieldElement],
) -> FieldElement {
    compute_hash_on_elements(&[
        PREFIX_CONTRACT_ADDRESS,
        FieldElement::ZERO,
        salt,
        class_hash,
        compute_hash_on_elements(constructor_calldata),
    ]) % ADDR_BOUND
}

pub fn is_number(s: &str) -> bool {
    is_hex(s) || is_decimal(s)
}

pub fn is_decimal(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }
    true
}

pub fn is_hex(s: &str) -> bool {
    if let Some(stripped) = s.strip_prefix("0x") {
        for c in stripped.chars() {
            if !c.is_ascii_hexdigit() {
                return false;
            }
        }
        true
    } else {
        false
    }
}

pub fn num_str_to_felt(number: &str) -> Result<FieldElement> {
    let context = || format!("Failed to parse felt from: `{}`", number);
    if number.starts_with("0x") {
        Ok(FieldElement::from_hex_be(number).with_context(context)?)
    } else {
        Ok(FieldElement::from_dec_str(number).with_context(context)?)
    }
}

pub fn short_str_to_felt(short_str: &str) -> Result<FieldElement> {
    let context = || format!("Failed to parse felt from short string: `{}`", short_str);
    Ok(FieldElement::from_byte_slice_be(short_str.as_bytes()).with_context(context)?)
}

pub fn udc_deployment_address(
    class_hash: FieldElement,
    mut salt: FieldElement,
    unique: bool,
    constructor_calldata: &[FieldElement],
    account_address: FieldElement,
) -> Result<FieldElement> {
    let mut deployer_for_address_generation = FieldElement::ZERO;
    if unique {
        salt = compute_hash_on_elements(&[account_address, salt]);
        deployer_for_address_generation = account_address;
    }

    Ok(compute_hash_on_elements(&[
        PREFIX_CONTRACT_ADDRESS,
        deployer_for_address_generation,
        salt,
        class_hash,
        compute_hash_on_elements(constructor_calldata),
    ]) % ADDR_BOUND)
}

#[test]
fn is_decimal_output() {
    assert!(!is_decimal("0x123"));
    assert!(!is_decimal("abc"));
    assert!(is_decimal("123"));
    assert!(!is_decimal("123k"));
}

#[test]
fn is_hex_output() {
    assert!(is_hex("0x123"));
    assert!(is_hex("0xabc"));
    assert!(!is_hex("123"));
    assert!(!is_hex("0xk"));
}
