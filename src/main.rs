//! the CLI wrapper around lib.rs

use clap::StructOpt;
use input::Format::{Json, Text};
use std::io;
use std::path::PathBuf;
use tikibase::{input, run, Message};

fn main() {
    let args = input::Arguments::parse();
    let result = run(args.command, PathBuf::from("."));
    match args.format {
        Text => print_text(&result.messages),
        Json => print_json(&result.messages),
    };
    std::process::exit(result.exit_code);
}

fn print_text(messages: &[Message]) {
    for message in messages {
        println!("{}", message.to_text());
    }
}

fn print_json(messages: &[Message]) {
    // NOTE: using a buffered writer doesn't seem to improve performance here
    if let Err(err) = serde_json::to_writer_pretty(io::stdout(), messages) {
        println!("Error serializing JSON: {}", err);
    }
}
