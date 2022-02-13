//! the CLI wrapper around lib.rs

use tikibase::process;
use tikibase::Command;

fn main() {
    let command = parse(std::env::args());
    let (mut outcomes, exitcode) = process(&command, ".");
    outcomes.sort();
    for outcome in outcomes {
        println!("{outcome}");
    }
    std::process::exit(exitcode);
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
    use tikibase::Command::{Check, Help, Stats, Version};

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
        for (give, want) in tests {
            let args = vec!["tikibase".into(), give.into()];
            let have = super::parse(args.into_iter());
            assert_eq!(have, want);
        }
    }
}
