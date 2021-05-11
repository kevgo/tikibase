extern crate lazy_static;

use std::path::PathBuf;
use tikibase::core::tikibase::Tikibase;
use tikibase::help;
use tikibase::probes;
use tikibase::stats;

fn main() {
    // step 1: determine the configuration
    let command = parse(std::env::args());

    // step 2: load the Tikibase
    let (base, errors) = Tikibase::load(PathBuf::from("."));
    for error in errors {
        println!("{}", error);
    }

    // step 3: execute the command
    let findings = match command {
        Command::Check => probes::run(base, false),
        Command::Fix => probes::run(base, true),
        Command::Help => help::run(),
        Command::Pitstop => probes::run(base, true),
        Command::Stats => stats::run(&base),
        Command::Version => help::version(),
    };

    // step 4: print the results
    for finding in findings {
        println!("{}", finding);
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Check,
    Fix,
    Help,
    Pitstop,
    Stats,
    Version,
}

/// Provides the command-line arguments as a Rust struct.
fn parse<I>(mut argv: I) -> Command
where
    I: Iterator<Item = String>,
{
    argv.next(); // skip argv[0]
    match argv.next() {
        None => Command::Help,
        Some(command) => match command.as_str() {
            "check" | "c" => Command::Check,
            "fix" | "f" => Command::Fix,
            "pitstop" | "ps" => Command::Pitstop,
            "stats" | "st" => Command::Stats,
            "version" | "v" => Command::Version,
            _ => Command::Help,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::Command::{Check, Help, Stats, Version};

    #[test]
    fn parse() {
        let tests = vec![
            ("check", Check),
            ("c", Check),
            ("stats", Stats),
            ("st", Stats),
            ("version", Version),
            ("v", Version),
            ("help", Help),
            ("h", Help),
            ("foo", Help),
        ];
        for (give, want) in tests.into_iter() {
            let args = vec!["tikibase".to_string(), give.to_string()];
            let have = super::parse(args.into_iter());
            assert_eq!(have, want);
        }
    }
}
