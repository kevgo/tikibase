extern crate lazy_static;

use std::path::PathBuf;
use tikibase::check;
use tikibase::fix;
use tikibase::help;
use tikibase::stats;

fn main() {
    let p = PathBuf::from(".");
    match parse(std::env::args()) {
        Command::Check => {
            for finding in check::run(p) {
                println!("{}", finding);
            }
        }
        Command::Fix => fix::run(p),
        Command::Help => help::run(),
        Command::Stats => stats::run(p),
        Command::Version => help::version(),
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Check,
    Fix,
    Help,
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
