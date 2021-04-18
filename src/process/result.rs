/// The result of a processor.
pub struct Result {
    pub findings: Vec<String>,
    pub fixes: Vec<String>,
}

impl Result {
    pub fn new() -> Result {
        Result {
            findings: vec![],
            fixes: vec![],
        }
    }
}

/// provides all given findings and fixes sorted alphabetically
pub struct SortedResults {
    items: Vec<String>,
}

impl SortedResults {
    pub fn new() -> SortedResults {
        SortedResults { items: Vec::new() }
    }

    /// registers the given results
    pub fn append(&mut self, items: &mut Result) {
        self.items.append(&mut items.findings);
        self.items.append(&mut items.fixes);
    }

    /// provides the registered results sorted alphabetically
    pub fn sorted(mut self) -> Vec<String> {
        self.items.sort();
        self.items
    }
}
