mod check;
mod help;
mod stats;
mod version;

pub(crate) use check::check;
pub(crate) use help::help;
pub(crate) use stats::stats;
pub(crate) use version::version;

#[derive(Debug, PartialEq)]
pub enum Command {
    Check,
    Fix,
    Help,
    Pitstop,
    Stats,
    Version,
}
