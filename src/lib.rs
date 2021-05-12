#![feature(map_into_keys_values)]
pub mod core;
pub mod help;
pub mod probes;
pub mod stats;
pub mod testhelpers;

use crate::core::tikibase::Tikibase;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum Command {
    Check,
    Fix,
    Help,
    Pitstop,
    Stats,
    Version,
}

pub fn process(command: Command, path: PathBuf) -> Vec<String> {
    let mut result = Vec::new();

    // step 1: load the Tikibase
    let (mut base, mut errors) = Tikibase::load(path);
    result.append(&mut errors);

    // step 2: basic command --> execute and exit
    let basic_command = match command {
        Command::Help => {
            help::run();
            true
        }
        Command::Stats => {
            stats::run(&base);
            true
        }
        Command::Version => {
            help::version();
            true
        }
        _ => false,
    };
    if basic_command {
        return result;
    }

    // here we have a complex command

    // step 3: find all issues in the Tikibase
    let issues = probes::run(&base);

    // step 4: take care of the issues
    let mut outcomes = match command {
        Command::Check => issues
            .into_iter()
            .map(|issue| issue.describe())
            .collect::<Vec<String>>(),
        Command::Fix => issues
            .into_iter()
            .filter(|issue| issue.fixable())
            .map(|issue| issue.fix(&mut base))
            .collect::<Vec<String>>(),
        Command::Pitstop => issues
            .into_iter()
            .map(|issue| {
                if issue.fixable() {
                    issue.fix(&mut base)
                } else {
                    issue.describe()
                }
            })
            .collect::<Vec<String>>(),
        _ => {
            panic!("unexpected complex command: {:?}", command);
        }
    };
    result.append(&mut outcomes);
    result
}
