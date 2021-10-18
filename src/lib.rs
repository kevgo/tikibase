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
    let mut outcomes = Vec::new();
    let mut exit_code = 0;
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
        return (outcomes, exit_code);
    }

    // load the Tikibase
    let (mut base, mut errors) = Tikibase::load(path, &config);
    exit_code += errors.len() as i32;
    outcomes.append(&mut errors);

    // handle stats command
    if command == &Command::Stats {
        commands::stats(&base);
        return (outcomes, exit_code);
    }

    // find all issues in the Tikibase
    let issues = commands::check(&base, &config);

    // take care of the issues
    match command {
        Command::Check => {
            for issue in issues {
                outcomes.push(issue.to_string());
                exit_code += 1;
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
                        outcomes.push(issue_desc);
                        exit_code += 1;
                    }
                }
            }
        }
        _ => {
            panic!("unexpected complex command: {:?}", command);
        }
    };
    (outcomes, exit_code)
}

// fn run_checks(issues: Vec<Box<dyn Issue>>) -> Vec<String> {
//     issues.iter().map(|issue| issue.to_string()).collect()
// }
