mod check;
pub mod commands;
pub mod config;
mod database;
mod fix;
pub mod input;
mod output;
pub mod test;

pub use config::Config;
use database::Tikibase;
pub use fix::Fix;
use input::Command;
pub use output::{Message, Messages};

/// runs the given Command in the given directory, returns structured data
#[must_use]
pub fn run(command: &input::Command, dir: &str) -> Messages {
  if command == &Command::Init {
    return Messages::from_outcome(commands::init(dir));
  }
  if command == &Command::JsonSchema {
    return Messages::from_outcome(commands::json_schema());
  }
  let mut base = match Tikibase::load(dir.into()) {
    Ok(base) => base,
    Err(issues) => return Messages::from_issues(issues),
  };
  let outcome = match command {
    Command::Check => commands::check(&base),
    Command::Stats => commands::stats(&base),
    Command::Fix => commands::fix(&mut base),
    Command::P => commands::pitstop(&mut base),
    Command::Init | Command::JsonSchema => panic!(), // handled above
  };
  Messages::from_outcome(outcome)
}
