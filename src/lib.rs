pub(crate) mod commands;
pub(crate) mod config;
pub(crate) mod database;
pub(crate) mod fixers;
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
    let mut unfix_count = 0;
    let mut outcomes: Vec<String> = Vec::new();

    // take care of the issues
    match command {
        Command::Check => {
            for issue in issues {
                outcomes.push(issue.to_string());
            }
        }
        Command::Fix => {
            for issue in issues.into_iter() {
                if let Some(fixer) = issue.fixer() {
                    outcomes.push(fixer.fix(&mut base, &config));
                }
            }
        }
        Command::Pitstop => {
            for issue in issues.into_iter() {
                let issue_desc = issue.to_string();
                match issue.fixer() {
                    Some(fixer) => {
                        outcomes.push(fixer.fix(&mut base, &config));
                    }
                    None => {
                        unfix_count += 1;
                        outcomes.push(issue_desc);
                    }
                }
            }
        }
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

// fn run_checks(issue: Vec<Box<dyn Issue>>) {
