use std::fmt::Display;

/// errors that are the user's fault and should be displayed to them
pub enum UserError {
  CannotWriteConfigFile { filename: String, reason: String },
}

impl Display for UserError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      UserError::CannotWriteConfigFile { filename, reason } => {
        write!(f, "cannot write config file {}: {}", filename, reason)
      }
    }
  }
}
