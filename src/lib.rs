pub mod cli;
mod commands;
mod config;
mod database;
mod fixers;
mod issue;
mod probes;
pub mod testhelpers;

pub use cli::Command;
use database::Tikibase;
use fixers::fix;
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
            // TODO: remove loop
            for issue in issues {
                outcomes.push(issue.to_string());
                exit_code += 1;
            }
        }
        Command::Fix => {
            for issue in issues {
                if let Some(fixed) = fix(&issue, &mut base, &config) {
                    outcomes.push(fixed);
                }
            }
        }
        Command::Pitstop => {
            for issue in issues {
                match fix(&issue, &mut base, &config) {
                    Some(fix_outcome) => outcomes.push(fix_outcome),
                    None => {
                        outcomes.push(issue.to_string());
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
