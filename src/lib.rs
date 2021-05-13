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

pub fn process<P: Into<PathBuf>>(command: Command, path: P) -> Vec<String> {
    let mut result = Vec::new();

    // step 1: load the Tikibase
    let (mut base, mut errors) = Tikibase::load(path.into());
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
    let mut outcomes: Vec<String> = match command {
        Command::Check => issues.into_iter().map(|issue| issue.describe()).collect(),
        Command::Fix => issues
            .into_iter()
            .filter(|issue| issue.fixable())
            .map(|fixable_issue| fixable_issue.fix(&mut base))
            .collect(),
        Command::Pitstop => issues
            .into_iter()
            .map(|issue| match issue.fixable() {
                true => issue.fix(&mut base),
                false => issue.describe(),
            })
            .collect(),
        _ => {
            panic!("unexpected complex command: {:?}", command);
        }
    };
    result.append(&mut outcomes);
    result
}
