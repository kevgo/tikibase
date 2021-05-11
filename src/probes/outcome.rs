use crate::core::tikibase::Tikibase;

/// an issue that was identified in the Tikibase
pub trait Issue {
    /// fixes this issue, returns a description of what it did
    fn fix(self, base: &mut Tikibase) -> String;

    /// indicates whether this issues is fixable
    fn fixable(&self) -> bool;

    /// provides a human-readable description of the issue
    fn describe(self) -> String;
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

// --------------------------------

/// The result of a processor.
pub struct Outcome {
    pub findings: Vec<String>,
    pub fixes: Vec<String>,
}

impl Outcome {
    pub fn new() -> Outcome {
        Outcome {
            findings: vec![],
            fixes: vec![],
        }
    }
}

/// provides all given findings and fixes sorted alphabetically
// TODO: make a type alias
pub struct SortedResults {
    items: Vec<String>,
}

impl SortedResults {
    pub fn new() -> SortedResults {
        SortedResults { items: Vec::new() }
    }

    /// registers the given results
    // TODO: take ownership of outcome
    pub fn append(&mut self, outcome: &mut Outcome) {
        self.items.append(&mut outcome.findings);
        self.items.append(&mut outcome.fixes);
    }

    /// provides the registered results sorted alphabetically
    pub fn sorted(mut self) -> Vec<String> {
        self.items.sort();
        self.items
    }
}
