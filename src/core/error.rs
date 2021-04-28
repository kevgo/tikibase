// use std::fmt;

#[derive(Debug)]
/// an error made by the user
pub struct UserError(pub String);

impl UserError {
    pub fn new(message: &str) -> UserError {
        UserError(message.to_string())
    }
}

// impl fmt::Display for UserError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", &self)
//     }
// }
