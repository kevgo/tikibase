pub mod commands;
pub mod config;
mod database;
mod fixers;
pub mod input;
mod output;
mod probes;
pub mod testhelpers;

pub use commands::{Issue, Outcome};
use database::Tikibase;
pub use fixers::Fix;
pub use output::{Message, Messages};
use std::path::PathBuf;

/// runs the given Command in the given directory, returns structured data
pub fn run(command: input::Command, dir: PathBuf) -> Messages {
    let config = match config::load(&dir) {
        Ok(config) => config,
        Err(issue) => return Messages::from_issue(issue),
    };
    let mut base = match Tikibase::load(dir, &config) {
        Ok(base) => base,
        Err(issues) => return Messages::from_issues(issues),
    };
    let outcome = match command {
        input::Command::Check => commands::check(&mut base, &config),
        input::Command::Stats => commands::stats(&base),
        input::Command::Fix => commands::fix(&mut base, &config),
        input::Command::Pitstop => commands::pitstop(&mut base, &config),
    };
    Messages::from_outcome(outcome)
}
