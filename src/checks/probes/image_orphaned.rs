use crate::config;

use crate::checks::{Issue, Issues};
use crate::database::Tikibase;

pub fn process(base: &Tikibase, resource_links: &[String]) -> Issues {
    let mut result = Issues::new();
    for resource in &base.resources {
        let path = resource.path.to_string_lossy();
        if !resource_links.iter().any(|rl| rl == &path) {
            result.push(Box::new(OrphanedResource { path: path.into() }));
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
