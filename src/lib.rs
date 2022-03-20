pub mod cli;
pub mod commands;
pub mod config;
mod database;
mod fix;
mod fixers;
mod issue;
mod message;
mod outcome;
mod probes;
pub mod testhelpers;

use database::Tikibase;
pub use fix::Fix;
pub use issue::Issue;
pub use message::{Message, Messages};
pub use outcome::Outcome;
use std::path::PathBuf;

/// runs the given Command in the given directory, returns structured data
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
        cli::Command::Check => commands::check(&mut base, &config),
        cli::Command::Stats => commands::stats(&base),
        cli::Command::Fix => commands::fix(&mut base, &config),
        cli::Command::Pitstop => commands::pitstop(&mut base, &config),
    };
    outcome.to_messages()
}
/// renders the given outcome into human-readable output
pub fn render_text(messages: Vec<Message>) -> (Vec<String>, i32) {
    let mut result: Vec<String> = vec![];
    let exit_code = messages.len() as i32;
    for message in messages {
        result.push(message.to_text());
    }
    result.sort();
    (result, exit_code)
}
