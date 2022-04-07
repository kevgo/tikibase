pub mod commands;
pub mod config;
mod database;
mod fix;
pub mod input;
mod output;
mod scan;
pub mod test;

pub use commands::{Issue, Location, Outcome};
pub use config::Config;
use database::Tikibase;
pub use fix::Fix;
use input::Command;
pub use output::{Message, Messages};
use std::path::PathBuf;

/// runs the given Command in the given directory, returns structured data
pub fn run(command: &input::Command, dir: PathBuf) -> Messages {
    if command == &Command::JsonSchema {
        return Messages::from_outcome(commands::json_schema());
    }
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
        Command::Init => commands::init(),
        Command::JsonSchema => panic!(), // handled above
        Command::Fix => commands::fix(&mut base, &config),
        Command::P => commands::pitstop(&mut base, &config),
    };
    Messages::from_outcome(outcome)
}
