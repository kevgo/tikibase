use crate::{config, core::tikibase::Tikibase};
use std::slice::Iter;

mod doc_links;
mod image_orphaned;
mod link_broken;
mod occurrences;
mod section_capitalization;
mod section_duplicate;
mod section_empty;
mod section_type;

pub fn run(base: &Tikibase, config: &config::Data) -> Issues {
    let mut issues = Issues::new();
    issues.append(section_duplicate::process(&base));
    issues.append(section_empty::process(&base));
    issues.append(section_capitalization::process(&base));
    issues.append(section_type::process(&base, &config));
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

    /// provides an iterator over borrowed references to the contained Issues
    pub fn iter(&self) -> Iter<Box<dyn Issue>> {
        self.0.iter()
    }

    /// indicates whether this collection contains any elements
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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

impl Default for Issues {
    fn default() -> Self {
        Issues::new()
    }
}

impl IntoIterator for Issues {
    type Item = Box<dyn Issue>;

    type IntoIter = std::vec::IntoIter<Box<dyn Issue>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
