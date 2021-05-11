use std::path::PathBuf;

use super::outcome::Issue;
use crate::core::tikibase::Tikibase;

/// finds all duplicate sections in the given Tikibase
pub fn process(base: &Tikibase) -> Vec<Box<dyn Issue>> {
    let mut result = Vec::<Box<dyn Issue>>::new();
    for doc in &base.docs {
        let mut known_sections = vec![];
        for section in &doc.content_sections {
            let section_type = section.section_type();
            if known_sections.contains(&section_type) {
                result.push(Box::new(DuplicateSection {
                    filename: doc.path.clone(),
                    section_type,
                }))
            } else {
                known_sections.push(section_type);
            }
        }
    }
    result
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

    fn fix(self, base: &mut Tikibase) -> String {
        panic!("not fixable");
    }

    fn describe(self) -> String {
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
    use crate::testhelpers;

    #[test]
    fn duplicate_sections() {
        let dir = testhelpers::tmp_dir();
        let content = "\
# test document

### One
content
### One
content";
        testhelpers::create_file("test.md", content, &dir);
        let (mut base, errs) = Tikibase::load(dir);
        assert_eq!(errs.len(), 0);
        let have = process(&mut base);
        assert_eq!(have.findings.len(), 1);
        assert_eq!(have.findings[0], "test.md  duplicate section: One");
        assert_eq!(have.fixes.len(), 0);
    }
}
