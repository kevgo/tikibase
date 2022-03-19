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
use serde::Serialize;
use std::path::PathBuf;

/// runs the given Command in the given directory, returns structured data
pub fn run(command: Command, dir: PathBuf) -> Outcome {
    let config = match config::load(&dir) {
        Ok(config) => config,
        Err(issue) => {
            return Outcome {
                issues: vec![issue],
                fixes: vec![],
            }
        }
    };
    let mut base = match Tikibase::load(dir, &config) {
        Ok(base) => base,
        Err(issues) => {
            return Outcome {
                issues,
                fixes: vec![],
            }
        }
    };
    match command {
        Command::Check => commands::check(&mut base, &config),
        Command::Stats => commands::stats(&base),
        Command::Fix => commands::fix(&mut base, &config),
        Command::Pitstop => commands::pitstop(&mut base, &config),
    }
}

/// result of running a Tikibase command
#[derive(Default, Serialize)]
pub struct Outcome {
    /// the issues identified but not fixed
    pub issues: Vec<Issue>,
    /// the fixes applied
    pub fixes: Vec<Fix>,
}

/// renders the given outcome into human-readable output
pub fn render_text(outcome: &Outcome) -> (Vec<String>, i32) {
    let mut result: Vec<String> = Vec::with_capacity(outcome.issues.len() + outcome.fixes.len());
    result.extend(outcome.issues.iter().map(|issue| issue.to_string()));
    result.extend(outcome.fixes.iter().map(|fix| fix.to_string()));
    result.sort();
    (result, outcome.issues.len() as i32)
}

/// the CLI args that this application accepts
#[derive(Debug, StructOpt)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// the command to run
    #[clap(subcommand)]
    pub command: Command,

    /// output format
    #[clap(arg_enum, long, short, default_value_t)]
    pub format: Format,
}

/// possible output formats
#[derive(clap::ArgEnum, Clone, Debug)]
pub enum Format {
    Text,
    Json,
}

/// the default output format
impl Default for Format {
    fn default() -> Self {
        Format::Text
    }
}

/// the subcommands of the CLI app
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
