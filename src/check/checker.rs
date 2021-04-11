use std::fmt;
use std::{collections::HashMap, path::Path};

/// an issue that occurs at a specific location (file, line)
pub trait LocalizedIssue {
    /// provides the location of the issue in the format "file:line"
    fn location(&self) -> String;
    /// provides a human-readable description of the issue
    fn desc(&self) -> String;
}

// a location in a tikibase
pub struct Location {
    /// the file that this issue occurs in
    pub file: String,
    /// line in the file, 0-based
    pub line: u32,
}

impl Location {
    pub fn from_path(filepath: &Path, line: u32) -> Location {
        Location {
            file: filepath.to_str().unwrap().to_string(),
            line,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}

pub struct LocalizedIssueCollector {
    pub issues: HashMap<String, Box<dyn LocalizedIssue>>,
}

impl LocalizedIssueCollector {
    pub fn new() -> LocalizedIssueCollector {
        LocalizedIssueCollector {
            issues: HashMap::new(),
        }
    }

    pub fn register(&mut self, issues: Vec<Box<dyn LocalizedIssue>>) {
        for issue in issues {
            self.issues.insert(issue.location(), issue);
        }
    }
}
