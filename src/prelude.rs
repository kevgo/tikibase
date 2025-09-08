//! stuff that is used in pretty much every file of this crate

use camino::Utf8PathBuf;
/// errors that are the user's fault and should be displayed to them
use core::fmt::Display;

#[derive(Eq, Debug, PartialEq)]
pub enum UserError {
  CannotWriteFile {
    filename: Utf8PathBuf,
    reason: String,
  },
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

/// a Result that always has a `UserError` as the error and therefore doesn't require to specify it at each call point
pub type Result<T> = core::result::Result<T, UserError>;
