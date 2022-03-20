//! the CLI wrapper around lib.rs

use clap::StructOpt;
use std::io;
use std::path::PathBuf;
use tikibase::{render_text, run, Message};

fn main() {
    let args = Args::parse();
    let messages = run(args.command, PathBuf::from("."));
    let exit_code = match args.format {
        Format::Text => print_text(messages),
        Format::Json => print_json(messages),
    };
    std::process::exit(exit_code);
}

fn print_text(messages: Vec<Message>) -> i32 {
    for message in messages {
        println!("{message.to_text()}");
    }
    messages.len() as i32
}

fn print_json(outcome: Outcome) -> i32 {
    // NOTE: using a buffered writer doesn't seem to improve performance here
    if let Err(err) = serde_json::to_writer_pretty(io::stdout(), &outcome) {
        println!("Error serializing JSON: {}", err);
        return 1;
    }
    outcome.issues.len() as i32
}
