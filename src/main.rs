//! the CLI wrapper around lib.rs

use clap::StructOpt;
use std::path::PathBuf;
use tikibase::{render_text, run, Args};

fn main() {
    let args = Args::parse();
    let (issues, fixes) = run(args.command, PathBuf::from("."));
    let (output, exit_code) = render_text(issues, fixes);
    for line in output {
        println!("{line}");
    }
    std::process::exit(exit_code);
}
