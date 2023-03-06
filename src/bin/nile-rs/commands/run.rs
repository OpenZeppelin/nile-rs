use super::CliCommand;

use std::process::{Command, ExitStatus};

use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Run {
    #[clap(help = "The script to execute.", value_name = "SCRIPT")]
    pub script: String,

    #[clap(help = "Arguments for the script.", value_name = "ARGS")]
    pub args: Vec<String>,
}

impl Run {
    fn exec(&self, mut command: Command) -> Result<ExitStatus> {
        command.status().with_context(|| {
            format!(
                "Unable to execute `{}`. \
                  Check if your script is located under the `./scripts` directory.",
                &self.script
            )
        })
    }
}

#[async_trait]
impl CliCommand for Run {
    type Output = ();

    /// Execute a script under the scripts directory
    async fn run(&self) -> Result<Self::Output> {
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

        self.exec(command)?;

        Ok(())
    }
}

#[test]
fn test_fail_to_exec() {
    let cli = Run {
        script: "declare".into(),
        args: vec![],
    };

    let command = Command::new("invalid");
    let error = cli.exec(command).unwrap_err();
    // Check top error or context
    assert_eq!(
        format!("{}", error),
        format!(
            "Unable to execute `{}`. \
            Check if your script is located under the `./scripts` directory.",
            &cli.script
        )
    );
}
