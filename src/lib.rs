pub mod cli;
pub mod commands;
pub mod config;
mod database;
mod fix;
mod output;
mod scan;
pub mod test;

use cli::Command;
pub use commands::{Issue, Outcome};
use database::Tikibase;
pub use fix::Fix;
pub use output::{Message, Messages};
use std::path::PathBuf;

/// runs the given Command in the given directory, returns structured data
// TODO: convert to INTO value so that we can give it &str
pub fn run(command: cli::Command, dir: PathBuf) -> Messages {
    let config = match config::load(&dir) {
        Ok(config) => config,
        Err(issue) => return Messages::from_issue(issue),
    };
    let mut base = match Tikibase::load(dir, &config) {
        Ok(base) => base,
        Err(issues) => return Messages::from_issues(issues),
    };
    let outcome = match command {
        Command::Check => commands::check(&mut base, &config),
        Command::Stats => commands::stats(&base),
        Command::Fix => commands::fix(&mut base, &config),
        Command::Pitstop => commands::pitstop(&mut base, &config),
    };
    Messages::from_outcome(outcome)
}
