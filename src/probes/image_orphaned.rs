use crate::database::Tikibase;
use crate::issue::Issue;

pub(crate) fn scan(base: &Tikibase, resource_links: &[String]) -> Vec<Issue> {
    let mut result = Vec::new();
    for resource in &base.resources {
        let path = resource.path.to_string_lossy();
        if !resource_links.iter().any(|rl| rl == &path) {
            result.push(Issue::OrphanedResource { path: path.into() });
        }
    }
    result
}
