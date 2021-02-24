use clap::Clap;

/// Tools for maintaining a Tikibase.
#[derive(Clap)]
struct Args {
  #[clap(subcommand)]
  pub command: Command,
}

#[derive(Clap)]
pub enum Command {
  /// Verifies the integrity of this Tikibase.
  /// Makes no changes.
  Check,
  /// Displays statistics about this Tikibase.
  Stats,
}

/// Parses the CLI args and provides the command that the user entered.
pub fn parse() -> Command {
  Args::parse().command
}
