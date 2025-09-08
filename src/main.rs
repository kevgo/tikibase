//! the CLI wrapper around lib.rs

use clap::StructOpt;
use input::Format::{Json, Text};
use std::io;
use std::process::ExitCode;
use tikibase::input::Command;
use tikibase::{Message, Messages, input, run};

fn main() -> ExitCode {
  match inner() {
    Ok(_) => ExitCode::SUCCESS,
    Err(err) => {
      println!("{}", err);
      ExitCode::FAILURE
    }
  }
}

fn inner() -> tikibase::Result<()> {
  let args = input::Arguments::parse();
  if args.command == Command::Init {
    return tikibase::commands::init(".");
  }
  if args.command == Command::JsonSchema {
    return tikibase::commands::json_schema();
  }
  let messages = run(args.command, ".");
  match args.format {
    Text => print_text(&messages, &args.command),
    Json => print_json(&messages.all()),
  };
  Ok(())
}

fn print_text(messages: &Messages, command: &Command) {
  if command != &Command::Fix {
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
