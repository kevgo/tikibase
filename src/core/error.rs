use std::error::Error;
use std::fmt;

#[derive(Debug)]
/// an error made by the user
pub struct UserError {
    message: String,
    guidance: String,
    source: dyn Error,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n\n{}", &self.message, &self.guidance)
    }
}

impl Error for UserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
