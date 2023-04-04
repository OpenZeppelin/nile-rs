use crate::{
    common::{devnet::get_predeployed_accounts, get_registered_accounts},
    core::accounts::OZAccount,
};
use anyhow::Result;

pub async fn get_accounts(network: &str, predeployed: bool) -> Result<Vec<OZAccount>> {
    if predeployed {
        get_predeployed_accounts(network).await
    } else {
        get_registered_accounts(network)
    }
}
