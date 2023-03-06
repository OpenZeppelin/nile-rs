use starknet_core::types::FieldElement;

/// Cairo string for "STARKNET_CONTRACT_ADDRESS"
pub const PREFIX_CONTRACT_ADDRESS: FieldElement = FieldElement::from_mont([
    3829237882463328880,
    17289941567720117366,
    8635008616843941496,
    533439743893157637,
]);

// Starknet address bound = 2 ** 251 - 256
pub const ADDR_BOUND: FieldElement = FieldElement::from_mont([
    18446743986131443745,
    160989183,
    18446744073709255680,
    576459263475590224,
]);
