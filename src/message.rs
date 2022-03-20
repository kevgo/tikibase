use crate::Issue;
use serde::Serialize;

/// a result struct of an activity, could be an issue of a fix
#[derive(Serialize)]
pub struct Message {
    pub file: Option<String>,
    pub line: Option<u32>,
    /// human-readable message
    pub text: String,
}

impl Message {
    pub fn to_text(self) -> String {
        match (self.file, self.line) {
            (Some(file), Some(line)) => {
                format!("{}:{}  {}", file, line, self.text)
            }
            (Some(file), None) => format!("{}  {}", file, self.text),
            (None, None) => self.text,
            (None, Some(_line)) => panic!("should never get just a line without a file"),
        }
    }
}

/// all activities
pub struct Messages {
    pub messages: Vec<Message>,
    pub exit_code: i32,
}

impl Messages {
    pub fn from_issue(issue: Issue) -> Messages {
        Messages {
            messages: vec![issue.to_message()],
            exit_code: 1,
        }
    }
    pub fn from_issues(issues: Vec<Issue>) -> Messages {
        Messages {
            exit_code: issues.len() as i32,
            messages: issues.into_iter().map(|issue| issue.to_message()).collect(),
        }
    }
}
