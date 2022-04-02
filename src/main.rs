//! the CLI wrapper around lib.rs

use clap::StructOpt;
use input::Format::{Json, Text};
use std::io;
use std::path::PathBuf;
use tikibase::{input, run, Message, Messages};

fn main() {
    let args = input::Arguments::parse();
    let messages = run(&args.command, PathBuf::from("."));
    match args.format {
        Text => print_text(&messages),
        Json => print_json(&messages.issues),
    };
    std::process::exit(messages.exit_code);
}

fn print_text(messages: &Messages) {
    if !messages.issues.is_empty() && !messages.fixes.is_empty() {
        println!("Issues:");
    }
    for issue in &messages.issues {
        println!("{}", issue.to_text());
    }
    if !messages.issues.is_empty() && !messages.fixes.is_empty() {
        println!("\nFixed:");
    }
    for fix in &messages.fixes {
        println!("{}", fix.to_text());
    }
}

fn print_json(messages: &[Message]) {
    // NOTE: using a buffered writer doesn't seem to improve performance here
    if let Err(err) = serde_json::to_writer_pretty(io::stdout(), messages) {
        println!("Error serializing JSON: {}", err);
    }
}
