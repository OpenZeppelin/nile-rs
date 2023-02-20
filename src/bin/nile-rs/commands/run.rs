use super::CliCommand;

use std::process::Command;

use anyhow::{Context, Ok, Result};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Run {
    #[clap(help = "The script to execute.", value_name = "SCRIPT")]
    pub script: String,

    #[clap(help = "Arguments for the script.", value_name = "ARGS")]
    pub args: Vec<String>,
}

impl CliCommand for Run {
    type Output = ();

    /// Execute a script under the scripts directory
    fn run(self) -> Result<Self::Output> {
        println!("Running {} script!", self.script);

        let mut command = Command::new("cargo");
        command.env("NILE_RS_TARGET_SCRIPT", &self.script);
        command.arg("run").arg("--release");

        if !self.args.is_empty() {
            command.arg("--");
            for arg in self.args.iter() {
                command.arg(arg);
            }
        }

        command.status().with_context(|| {
            format!(
                "Unable to execute `{}`. \
                      Check if your script is located under the `./scripts` directory.",
                &self.script
            )
        })?;

        Ok(())
    }
}
