mod check;
pub mod commands;
pub mod config;
mod database;
pub mod errors;
mod fix;
pub mod input;
mod output;
pub mod prelude;
pub mod test;

pub use config::Config;
use database::Tikibase;
pub use errors::UserError;
pub use fix::Fix;
use input::Command;
pub use output::{Message, Messages};
pub use prelude::Result;

// TODO
// - replace Utf8Paths with Path
// - use UserError everywhere
// - extract string literals into consts

/// runs the given Command in the given directory, returns structured data
#[must_use]
pub fn run(command: input::Command, dir: &str) -> Messages {
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
