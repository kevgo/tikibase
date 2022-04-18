use std::ffi::OsString;
use ahash::AHashMap;
use crate::database::Directory;
use crate::{Issue, Tikibase};

pub(crate) fn scan(base: &Tikibase, resource_links: &[String]) -> Vec<Issue> {
    let mut result = Vec::new();

    // plan:
    // - create a Trie with referenced resources
    // - create a Trie with existing resources (exists already)
    // - referenced - existing --> dead resource links
    // - existing - referenced --> unlinked resources
    let all_resources = base.all_resources();
    // normalized path --> used/not used
    let mut resource_is_used : AHashMap<&OsString, bool> = AHashMap::with_capacity(all_resources.len());
    for resource in all_resources {
        resource_is_used.insert(resource, false);
    }
    scan_dir(&base.dir, "/".into(), &mut resource_is_used);
}

fn scan_dir(dir: &Directory, path_to_dir: OsString, resource_is_used: &mut AHashMap<&OsString, bool>) {
    for document in dir.documents {
        if base.has_resource(doc, link)
        let path = resource.path.to_string_lossy();
        if !resource_links.iter().any(|rl| rl == &path) {
            result.push(Issue::OrphanedResource {
                location: Location {
                    file: resource.path.clone(),
                    line: 0,
                    start: 0,
                    end: 0,
                },
            });
        }
    }
    result
