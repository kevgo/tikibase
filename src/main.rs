mod args;
mod check;
mod core;
mod help;
mod stats;

use args::Command::{Check, Help, Stats, Version};

fn main() {
    match args::parse(std::env::args()) {
        Check => check::run(),
        Help => help::run(),
        Stats => stats::run(),
        Version => help::version(),
    }
}
