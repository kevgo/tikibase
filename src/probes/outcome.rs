use crate::core::tikibase::Tikibase;

/// an issue that was identified in the Tikibase
pub trait Issue {
    /// fixes this issue, returns a description of what it did
    fn fix(&self, base: &mut Tikibase) -> String;

    /// indicates whether this issues is fixable
    fn fixable(&self) -> bool;

    /// provides a human-readable description of the issue
    fn describe(&self) -> String;
}

/// a sorted list of issues
pub struct Issues(Vec<Box<dyn Issue>>);

impl Issues {
    /// appends the given issue to this issue list
    pub fn append(&mut self, mut new_issues: Vec<Box<dyn Issue>>) {
        self.0.append(&mut new_issues);
    }

    /// provides an empty issue list
    pub fn new() -> Issues {
        Issues(vec![])
    }

    pub fn issues(self) -> Vec<Box<dyn Issue>> {
        self.0
    }
}
