use core::fmt::Display;

/// errors that are the user's fault and should be displayed to them
#[derive(Debug)]
pub enum UserError {
  CannotWriteFile { filename: String, reason: String },
}

impl Display for UserError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      UserError::CannotWriteFile { filename, reason } => {
        write!(f, "cannot write file {}: {}", filename, reason)
      }
    }
  }
}
