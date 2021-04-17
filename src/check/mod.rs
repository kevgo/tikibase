use crate::core::tikibase::Tikibase;
use std::path::PathBuf;
mod empty_sections;
mod section_capitalization;

pub fn run(dir: PathBuf) -> Vec<String> {
    let base = Tikibase::in_dir(dir);
    let mut issues = Vec::new();
    issues.append(&mut empty_sections::find(&base));
    issues.append(&mut section_capitalization::check(&base));
    issues.sort();
    issues
}
