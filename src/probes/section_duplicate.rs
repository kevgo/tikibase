use std::path::PathBuf;

use super::{Issue, Issues};
use crate::config;
use crate::core::tikibase::Tikibase;

/// finds all duplicate sections in the given Tikibase
pub fn process(base: &Tikibase) -> Issues {
    let mut issues = Issues::new();
    for doc in &base.docs {
        let mut known_sections = Vec::new();
        for section in &doc.content_sections {
            let section_type = section.section_type();
            if known_sections.contains(&section_type) {
                issues.push(Box::new(DuplicateSection {
                    filename: doc.path.clone(),
                    section_type: section_type.into(),
                }))
            } else {
                known_sections.push(section_type);
            }
        }
    }
    issues
}

/// describes the issue that a document contains two sections with the same title
pub struct DuplicateSection {
    filename: PathBuf,
    section_type: String,
}

impl Issue for DuplicateSection {
    fn fixable(&self) -> bool {
        false
    }

    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        panic!("not fixable");
    }

    fn describe(&self) -> String {
        format!(
            "{}  duplicate section: {}",
            self.filename.to_string_lossy(),
            self.section_type
        )
    }
}

#[cfg(test)]
mod tests {

    use super::process;
    use crate::core::tikibase::Tikibase;
    use crate::testhelpers::{create_file, empty_config, tmp_dir};

    #[test]
    fn duplicate_sections() {
        let dir = tmp_dir();
        let content = "\
# test document

### One
content
### One
content";
        create_file("test.md", content, &dir);
        let (base, errs) = Tikibase::load(dir, &empty_config());
        assert_eq!(errs.len(), 0);
        let have: Vec<String> = process(&base)
            .iter()
            .map(|issue| issue.describe())
            .collect();
        assert_eq!(have.len(), 1);
        assert_eq!(have[0], "test.md  duplicate section: One");
    }
}
