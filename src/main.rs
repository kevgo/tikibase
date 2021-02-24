mod check;
mod cli;
mod core;
mod stats;

use cli::Command::{Check, Stats};

fn main() {
    match cli::parse_args() {
        Check => check::run(),
        Stats => stats::run(),
    }
}
