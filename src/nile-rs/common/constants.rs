use starknet_crypto::FieldElement;

/// The default UDC address: 0x041a78e741e5af2fec34b695679bc6891742439f7afb8484ecd7766661ad02bf.
pub const UDC_ADDRESS: FieldElement = FieldElement::from_mont([
    15144800532519055890,
    15685625669053253235,
    9333317513348225193,
    121672436446604875,
]);

/// Selector for entrypoint `deployContract`.
pub const SELECTOR_DEPLOYCONTRACT: FieldElement = FieldElement::from_mont([
    18249998464715511309,
    1265649739554438882,
    1439621915307882061,
    469988280392664069,
]);
