mod check;
mod help;
mod stats;

pub(crate) use check::check;
pub(crate) use help::{help, version};
pub(crate) use stats::stats;

#[derive(Debug, PartialEq)]
pub enum Command {
    Check,
    Fix,
    Help,
    Pitstop,
    Stats,
    Version,
}
