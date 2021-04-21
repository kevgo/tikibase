use std::fmt;

#[derive(Debug)]
/// an error made by the user
pub struct UserError {
    message: String,
    guidance: String,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n\n{}", &self.message, &self.guidance)
    }
}

impl From<std::io::Error> for UserError {
    fn from(error: std::io::Error) -> Self {
        UserError {
            message: error.to_string(),
            guidance: "Please make sure that all files and directories are accessible.".to_string(),
        }
    }
}
