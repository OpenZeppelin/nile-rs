use snapbox::cmd::cargo_bin;
use snapbox::cmd::Command as SnapboxCommand;
use std::process::Command as StdCommand;

pub fn snapbox() -> SnapboxCommand {
    SnapboxCommand::from_std(std())
}

fn std() -> StdCommand {
    StdCommand::new(cargo_bin("nile-rs"))
}
