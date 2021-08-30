use crate::database::Tikibase;
use crate::{issues, Issues};

pub fn process(base: &Tikibase, resource_links: &[String]) -> Issues {
    let mut result = Issues::new();
    for resource in &base.resources {
        let path = resource.path.to_string_lossy();
        if !resource_links.iter().any(|rl| rl == &path) {
            result.push(Box::new(issues::OrphanedResource { path: path.into() }));
        }
    }
    result
}
