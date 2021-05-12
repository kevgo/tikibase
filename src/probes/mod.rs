use crate::core::tikibase::Tikibase;
use std::slice::Iter;
use std::vec::IntoIter;

mod doc_links;
mod image_orphaned;
mod link_broken;
mod occurrences;
mod section_capitalization;
mod section_duplicate;
mod section_empty;

pub fn run(base: &Tikibase) -> Issues {
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
    issues
}

/// an issue that was identified in the Tikibase
pub trait Issue {
    /// provides a human-readable description of the issue
    fn describe(&self) -> String;

    /// fixes this issue, returns a human-readable description of what it did
    fn fix(&self, base: &mut Tikibase) -> String;

    /// indicates whether this issue is fixable
    fn fixable(&self) -> bool;
}

/// a collection of issues
pub struct Issues(Vec<Box<dyn Issue>>);

impl Issues {
    /// appends the given issue to this issue collection
    pub fn append(&mut self, mut new_issues: Issues) {
        self.0.append(&mut new_issues.0);
    }

    /// consumes this Issue collection into an iterator
    pub fn into_iter(self) -> IntoIter<Box<dyn Issue>> {
        self.0.into_iter()
    }

    /// provides an iterator over borrowed references to the contained Issues
    pub fn iter(&self) -> Iter<Box<dyn Issue>> {
        self.0.iter()
    }

    /// provides the number of issues in this Issue collection
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// provides an empty Issues instance
    pub fn new() -> Issues {
        Issues(vec![])
    }

    /// adds the given Issue to this Issue collection
    pub fn push(&mut self, issue: Box<dyn Issue>) {
        self.0.push(issue);
    }
}
