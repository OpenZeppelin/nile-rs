use anyhow::{anyhow, Context, Ok, Result};
use serde::{Deserialize, Serialize};

use crate::config::Config;

const TO_REPLACE: &str = "<network>";
const FILE_NAME_FORMAT: &str = "<network>.accounts.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub name: String,
    pub address: String,
    pub public_key: String,
}

impl AccountInfo {
    /// Attempt to get the account data from the file system
    pub fn load_from_signer(private_key_env: &str, network: &str) -> Result<Self> {
        let config = Config::get()?;
        let db_file_name = [
            &config.deployments_dir,
            "/",
            &FILE_NAME_FORMAT.replace(TO_REPLACE, network),
        ]
        .concat();

        let context = || {
            format!(
                "Failed to load the account from: `{}`",
                db_file_name.replace("//", "/")
            )
        };
        let accounts: Vec<AccountInfo> =
            serde_json::from_reader(std::fs::File::open(&db_file_name).with_context(context)?)
                .with_context(context)?;

        let result = accounts.into_iter().find(|acc| acc.name == private_key_env);

        match result {
            Some(acc) => Ok(acc),
            None => Err(anyhow!(
                "Account not found! If the account is deployed \
                already, try registering it in `{}`",
                db_file_name.replace("//", "/")
            )),
        }
    }

    /// Attempt to save the account data in the file system
    pub fn save(
        private_key_env: &str,
        address: &str,
        public_key: &str,
        network: &str,
    ) -> Result<()> {
        let config = Config::get()?;
        let db_file_name = [
            &config.deployments_dir,
            "/",
            &FILE_NAME_FORMAT.replace(TO_REPLACE, network),
        ]
        .concat();

        // Ensure the directories exists
        std::fs::create_dir_all(&config.deployments_dir)?;

        let new_account = AccountInfo {
            name: private_key_env.into(),
            address: address.into(),
            public_key: public_key.into(),
        };

        let mut accounts: Vec<AccountInfo> = vec![];
        if std::path::Path::new(&db_file_name).exists() {
            accounts = serde_json::from_reader(std::fs::File::open(&db_file_name)?).with_context(
                || {
                    format!(
                        "Failed to load the existing accounts from `{}`.\n\
                        Try removing the file if it is empty.",
                        db_file_name.replace("//", "/")
                    )
                },
            )?;
        };

        accounts.push(new_account);

        // Save the JSON structure into the file.
        std::fs::write(
            db_file_name,
            serde_json::to_string_pretty(&accounts).unwrap(),
        )?;

        Ok(())
    }
}

#[test]
fn error_context() {
    let error = AccountInfo::load_from_signer("invalid_name", "localhost").unwrap_err();
    assert_eq!(
        format!("{}", error),
        "Failed to load the account from: `deployments/localhost.accounts.json`"
    );
}
