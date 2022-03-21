use crate::{Issue, Message, Outcome};

#[derive(Debug, Default, PartialEq)]
pub struct Messages {
    pub messages: Vec<Message>,
    pub exit_code: i32,
}

impl Messages {
    pub fn from_issue(issue: Issue) -> Messages {
        Messages {
            messages: vec![Message::from_issue(issue)],
            exit_code: 1,
        }
    }
    pub fn from_issues(issues: Vec<Issue>) -> Messages {
        Messages {
            exit_code: issues.len() as i32,
            messages: issues.into_iter().map(Message::from_issue).collect(),
        }
    }

    pub fn from_outcome(outcome: Outcome) -> Messages {
        let exit_code = outcome.issues.len() as i32;
        let mut messages = vec![];
        messages.extend(outcome.fixes.into_iter().map(Message::from_fix));
        messages.extend(outcome.issues.into_iter().map(Message::from_issue));
        Messages {
            messages,
            exit_code,
        }
    }
}
