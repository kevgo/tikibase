//! the CLI wrapper around lib.rs

use clap::StructOpt;
use std::io;
use std::path::PathBuf;
use tikibase::{cli, run, Message, Messages};

fn main() {
    let args = cli::Args::parse();
    let Messages {
        messages,
        exit_code,
    } = run(args.command, PathBuf::from("."));
    match args.format {
        cli::Format::Text => print_text(messages),
        cli::Format::Json => print_json(messages),
    };
    std::process::exit(exit_code);
}

fn print_text(messages: Vec<Message>) {
    for message in messages {
        println!("{}", message.to_text());
    }
}

fn print_json(messages: Vec<Message>) {
    // NOTE: using a buffered writer doesn't seem to improve performance here
    if let Err(err) = serde_json::to_writer_pretty(io::stdout(), &messages) {
        println!("Error serializing JSON: {}", err);
    }
}
