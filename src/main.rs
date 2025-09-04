//! the CLI wrapper around lib.rs

use clap::StructOpt;
use input::Format::{Json, Text};
use std::io;
use std::process::ExitCode;
use tikibase::input::Command;
use tikibase::{Message, Messages, input, run};

fn main() -> ExitCode {
  let args = input::Arguments::parse();
  let messages = run(args.command.clone(), ".");
  let exit_code = messages.exit_code;
  match args.format {
    Text => print_text(&messages, &args.command),
    Json => print_json(&messages.all()),
  };
  ExitCode::from(exit_code)
}

fn print_text(messages: &Messages, command: &Command) {
  if !matches!(command, Command::Fix) {
    for issue in &messages.issues {
      println!("{}", issue.to_text());
    }
  }
  if messages.has_issues_and_fixes() {
    println!();
  }
  for fix in &messages.fixes {
    println!("{}", fix.to_text());
  }
}

fn print_json(messages: &[Message]) {
  // NOTE: using a buffered writer doesn't seem to improve performance here
  if let Err(err) = serde_json::to_writer_pretty(io::stdout(), messages) {
    println!("Error serializing JSON: {err}");
  }
}
