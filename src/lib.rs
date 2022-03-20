pub mod commands;
pub mod config;
mod database;
mod fixers;
mod issues;
mod probes;
pub mod testhelpers;

use clap::StructOpt;
use database::Tikibase;
pub use fixers::Fix;
pub use issues::Issue;
use std::path::PathBuf;

/// runs the given Command in the given directory, returns structured data
pub fn run(command: Command, dir: PathBuf) -> (Vec<Issue>, Vec<Fix>) {
    let config = match config::load(&dir) {
        Ok(config) => config,
        Err(issue) => return (vec![issue], vec![]),
    };
    let mut base = match Tikibase::load(dir, &config) {
        Ok(base) => base,
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
    let mut result: Vec<String> = Vec::with_capacity(issues.len() + fixes.len());
    result.extend(issues.iter().map(|issue| issue.to_string()));
    result.extend(fixes.iter().map(|fix| fix.to_string()));
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
    /// Finds and prints issues, does not make changes
    Check,
    /// Corrects all auto-fixable issues
    Fix,
    /// Corrects all auto-fixable issues, prints all remaining issues
    Pitstop,
    /// Displays statistics about this Tikibase
    Stats,
}
