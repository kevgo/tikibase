use crate::checks::{issues, Issues};
use crate::database::Tikibase;

/// finds all duplicate sections in the given Tikibase
pub fn process(base: &Tikibase) -> Issues {
    let mut issues = Issues::new();
    for doc in &base.docs {
        let mut known_sections = Vec::new();
        for section in &doc.content_sections {
            let section_type = section.section_type();
            if known_sections.contains(&section_type) {
                issues.push(Box::new(issues::DuplicateSection {
                    filename: doc.path.clone(),
                    section_type: section_type.into(),
                }));
            } else {
                known_sections.push(section_type);
            }
        }
    }
    issues
}

#[cfg(test)]
mod tests {

    use super::process;
    use crate::database::Tikibase;
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
