use clap::StructOpt;

/// Linter for Markdown-based knowledge databases
#[derive(Debug, StructOpt)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// the command to run
    #[clap(subcommand)]
    pub command: Command,

    /// Output format
    #[clap(arg_enum, long, short, default_value_t)]
    pub format: Format,
}

/// possible output formats for the CLI app
#[derive(clap::ArgEnum, Clone, Debug)]
pub enum Format {
    Text,
    Json,
}

/// the default output format of the CLI app
impl Default for Format {
    fn default() -> Self {
        Format::Text
    }
}

/// the subcommands of the CLI app
#[derive(Debug, PartialEq, clap::Subcommand)]
pub enum Command {
    /// Finds and prints issues, does not make changes
    Check,
    /// Corrects all auto-fixable issues
    Fix,
    /// Corrects all auto-fixable issues, prints all remaining issues
    Pitstop,
    /// Displays statistics about this Tikibase
    Stats,
}
