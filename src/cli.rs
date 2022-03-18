use clap::StructOpt;

/// Linter for Markdown-based semantic knowledge-bases
#[derive(Debug, StructOpt)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// the command to run
    #[clap(subcommand)]
    pub command: Command,

    /// output format
    #[clap(arg_enum, long, short, default_value_t)]
    pub format: Format,
}

#[derive(Debug, PartialEq, clap::Subcommand)]
pub enum Command {
    /// The command to run on CI. Finds and prints issues, does not make changes.
    Check,
    /// corrects all auto-fixable issues
    Fix,
    /// Corrects all auto-fixable issues, prints all remaining issues.
    Pitstop,
    /// displays statistics about this Tikibase
    Stats,
}

#[derive(clap::ArgEnum, Clone, Debug)]
pub enum Format {
    Text,
    Json,
}

impl Default for Format {
    fn default() -> Self {
        Format::Text
    }
}
