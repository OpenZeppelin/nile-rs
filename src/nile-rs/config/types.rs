use serde::{Deserialize, Serialize};
use starknet_crypto::FieldElement;

use crate::common::str_to_felt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    pub name: String,
    pub gateway: String,
    pub feeder_gateway: Option<String>,
    pub chain_id: String,
}

impl Network {
    pub fn normalized_feeder_gateway(&self) -> String {
        if self.feeder_gateway.is_none() {
            self.gateway.replace("gateway", "feeder_gateway")
        } else {
            self.feeder_gateway.clone().unwrap()
        }
    }

    pub fn chain_id_in_felt(&self) -> FieldElement {
        str_to_felt(&self.chain_id)
    }

    pub fn predeployed_accounts_endpoint(&self) -> String {
        self.gateway.replace("gateway", "predeployed_accounts")
    }
}

#[test]
fn feeder_gateway() {
    let network = Network {
        name: "localhost".into(),
        gateway: "/gateway".into(),
        chain_id: "0".into(),
        feeder_gateway: None,
    };

    assert_eq!(network.normalized_feeder_gateway(), "/feeder_gateway");
    assert_eq!(network.feeder_gateway, None);
}
