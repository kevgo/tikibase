use crate::{Issue, Location, Tikibase};
use std::ffi::OsStr;
use std::path::PathBuf;

pub(crate) fn scan(base: &Tikibase, resource_links: &[String]) -> Vec<Issue> {
    let mut result = Vec::new();
    for resource in base.dir.resources.keys() {
        // TODO: maybe make resource_links an AHashMap?
        if !resource_links.iter().any(|rl| OsStr::new(rl) == resource) {
            result.push(Issue::OrphanedResource {
                location: Location {
                    file: PathBuf::from(resource),
                    line: 0,
                    start: 0,
                    end: 0,
                },
            });
        }
    }
    result
}
