pub mod constants;
pub mod devnet;
pub mod fs;

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
