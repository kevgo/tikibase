pub mod commands;
pub mod config;
mod database;
mod fixers;
mod issues;
mod probes;
pub mod testhelpers;

use clap::StructOpt;
use database::open;
use database::Tikibase;
pub use fixers::Fix;
pub use issues::Issue;
use std::path::PathBuf;

/// runs the given Command in the given directory, returns structured data
pub fn run(command: Command, dir: PathBuf) -> (Vec<Issue>, Vec<Fix>) {
    let (mut base, config) = match crate::open(dir) {
        Ok(data) => data,
        Err(issues) => return (issues, vec![]),
    };
    match command {
        Command::Check => commands::check(&mut base, &config),
        Command::Stats => commands::stats(&base),
        Command::Fix => commands::fix(&mut base, &config),
        Command::Pitstop => commands::pitstop(&mut base, &config),
    }
}

/// renders the given issues and fixes into human-readable output
pub fn render_text(issues: Vec<Issue>, fixes: Vec<Fix>) -> (Vec<String>, i32) {
    let mut result: Vec<String> = vec![];
    for issue in &issues {
        result.push(issue.to_string())
    }
    for fix in fixes {
        result.push(fix.to_string())
    }
    result.sort();
    (result, issues.len() as i32)
}

/// the CLI args that this application accepts
#[derive(Debug, StructOpt)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// the command to run
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, PartialEq, clap::Subcommand)]
pub enum Command {
    /// The command to run on CI. Finds and prints issues, does not make changes.
    Check,
    /// corrects all auto-fixable issues
    Fix,
    /// Corrects all auto-fixable issues, prints all remaining issues.
    Pitstop,
    /// displays statistics about this Tikibase
    Stats,
}
