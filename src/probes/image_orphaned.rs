use super::outcome::Outcome;
use super::Tikibase;

pub fn process(base: &Tikibase, resource_links: Vec<String>) -> Outcome {
    let mut result = Outcome::new();
    for resource in base.resources.iter() {
        let path = &resource.path.to_string_lossy().to_string();
        if !resource_links.contains(&path) {
            result.findings.push(format!("unused image \"{}\"", path));
        }
    }
    result
}