use crate::{Fix, Issue, Messages};
use serde::Serialize;

/// result of running a Tikibase command
#[derive(Default, Serialize)]
pub struct Outcome {
    /// the issues identified but not fixed
    pub issues: Vec<Issue>,
    /// the fixes applied
    pub fixes: Vec<Fix>,
}

impl Outcome {
    pub fn to_messages(self) -> Messages {
        let mut messages = vec![];
        let exit_code = self.issues.len() as i32;
        messages.extend(self.fixes.into_iter().map(|fix| fix.to_message()));
        messages.extend(self.issues.into_iter().map(|issue| issue.to_message()));
        Messages {
            messages,
            exit_code,
        }
    }
}
