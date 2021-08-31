#![feature(cow_is_borrowed)]

pub(crate) mod commands;
pub(crate) mod config;
pub(crate) mod database;
pub(crate) mod issues;
pub(crate) mod probes;
pub mod testhelpers;

pub use commands::Command;
use database::Tikibase;
use issues::Issue;
use std::path::PathBuf;

pub fn process<P: Into<PathBuf>>(command: &Command, path: P) -> (Vec<String>, i32) {
    let mut result = Vec::new();
    let path = path.into();

    // load the configuration
    let config = match config::load(&path) {
        Ok(config) => config,
        Err(err) => return (vec![err], 1),
    };

    // handle non-repo commands
    let basic_command = match command {
        Command::Help => {
            commands::help();
            true
        }
        Command::Version => {
            commands::version();
            true
        }
        _ => false,
    };
    if basic_command {
        return (result, 0);
    }

    // load the Tikibase
    let (mut base, mut errors) = Tikibase::load(path, &config);
    result.append(&mut errors);

    // handle stats command
    if command == &Command::Stats {
        commands::stats(&base);
        return (result, 0);
    }

    // find all issues in the Tikibase
    let issues = commands::check(&base, &config);
    let unfix_count = issues.iter().filter(|issue| !issue.fixable()).count() as i32;

    // take care of the issues
    let mut outcomes: Vec<String> = match command {
        Command::Check => issues.into_iter().map(|issue| issue.describe()).collect(),
        Command::Fix => issues
            .into_iter()
            .filter(|issue| issue.fixable())
            .map(|fixable_issue| fixable_issue.fix(&mut base, &config))
            .collect(),
        Command::Pitstop => issues
            .into_iter()
            .map(|issue| match issue.fixable() {
                true => issue.fix(&mut base, &config),
                false => issue.describe(),
            })
            .collect(),
        _ => {
            panic!("unexpected complex command: {:?}", command);
        }
    };
    let exitcode = match command {
        Command::Check => outcomes.len() as i32,
        Command::Fix => 0,
        Command::Pitstop => unfix_count,
        _ => 0,
    };
    result.append(&mut outcomes);
    (result, exitcode)
}
