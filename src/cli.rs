use clap::StructOpt;

#[derive(Debug, StructOpt)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// the command to run
    #[clap(subcommand)]
    pub command: Command,
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
