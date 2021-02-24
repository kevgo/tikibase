mod cli;

use cli::Command::{Check, Stats};

fn main() {
    match cli::parse() {
        Check => println!("checking"),
        Stats => println!("statistics"),
    }
}
