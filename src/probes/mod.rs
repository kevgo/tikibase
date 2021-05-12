use crate::core::tikibase::Tikibase;
mod doc_links;
mod image_orphaned;
mod link_broken;
mod occurrences;
mod section_capitalization;
mod section_duplicate;
mod section_empty;

pub fn run(base: &Tikibase) -> Vec<Box<dyn Issue>> {
    let mut issues = Issues::new();
    issues.append(section_duplicate::process(&base));
    issues.append(section_empty::process(&base));
    issues.append(section_capitalization::process(&base));
    let links_result = link_broken::process(&base);
    issues.append(links_result.issues);
    issues.append(image_orphaned::process(
        &base,
        links_result.outgoing_resource_links,
    ));
    let occ_res = occurrences::process(
        base,
        &links_result.incoming_doc_links,
        &links_result.outgoing_doc_links,
    );
    issues.append(occ_res);
    // issues.sorted()
    issues.issues()
}

/// an issue that was identified in the Tikibase
pub trait Issue {
    /// provides a human-readable description of the issue
    fn describe(&self) -> String;

    /// fixes this issue, returns a description of what it did
    fn fix(&self, base: &mut Tikibase) -> String;

    /// indicates whether this issues is fixable
    fn fixable(&self) -> bool;
}

/// a sorted list of issues
pub struct Issues(Vec<Box<dyn Issue>>);

impl Issues {
    /// appends the given issue to this issue list
    pub fn append(&mut self, mut new_issues: Vec<Box<dyn Issue>>) {
        self.0.append(&mut new_issues);
    }

    /// provides the issues stored in this custom class
    pub fn issues(self) -> Vec<Box<dyn Issue>> {
        self.0
    }

    /// provides an empty Issues instance
    pub fn new() -> Issues {
        Issues(vec![])
    }
}
