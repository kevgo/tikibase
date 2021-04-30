// use std::fmt;

#[derive(Debug)]
/// an error made by the user
pub struct UserError(pub String);

#[derive(Debug)]
pub enum Outcome {
    /// describes an issue that the user has to fix
    UserError(String),
    /// describes an activity that this app performs
    Notification(String),
}

pub type Outcomes = Vec<Outcome>;

// impl fmt::Display for Outcome {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", &self.0)
//     }
// }
