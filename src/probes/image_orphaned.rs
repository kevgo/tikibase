use crate::config;

use super::Tikibase;
use super::{Issue, Issues};

pub fn process(base: &Tikibase, resource_links: Vec<String>) -> Issues {
    let mut result = Issues::new();
    for resource in base.resources.iter() {
        let path = resource.path.to_string_lossy().to_string();
        if !resource_links.contains(&path) {
            result.push(Box::new(OrphanedResource { path }));
        }
    }
    result
}

/// a resource that isn't linked to
pub struct OrphanedResource {
    path: String,
}

impl Issue for OrphanedResource {
    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        panic!("not fixable")
    }

    fn fixable(&self) -> bool {
        false
    }

    fn describe(&self) -> String {
        format!("unused resource \"{}\"", self.path)
    }
}

// TODO: add tests
