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
