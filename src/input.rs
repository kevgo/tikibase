//! The arguments that can be provided to Tikibase via CLI.

/// Linter for Markdown-based knowledge databases
#[derive(Debug, clap::StructOpt)]
#[clap(version, about, long_about = None)]
pub struct Arguments {
  /// the command to run
  #[clap(subcommand)]
  pub command: Command,

  /// Output format
  #[clap(arg_enum, long, short, default_value_t)]
  pub format: Format,
}

/// possible output formats for the CLI app
#[derive(clap::ArgEnum, Clone, Copy, Debug)]
pub enum Format {
  Text,
  Json,
}

/// the default output format of the CLI app
impl Default for Format {
  fn default() -> Self {
    Self::Text
  }
}

/// the subcommands of the CLI app
#[derive(clap::Subcommand, Clone, Copy, Debug, PartialEq)]
pub enum Command {
  /// Prints all issues
  Check,
  /// Corrects all auto-fixable issues
  Fix,
  /// Scaffolds a configuration file
  Init,
  /// Export the JSON Schema for the configuration file
  JsonSchema,
  /// Corrects all auto-fixable issues and prints the remaining ("pitstop")
  P,
  /// Displays statistics about this Tikibase
  Stats,
}
