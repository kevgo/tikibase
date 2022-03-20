use crate::{Issue, Tikibase};

/// finds all duplicate sections in the given Tikibase
pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::new();
    for doc in &base.docs {
        let mut known_sections = Vec::new();
        for section in &doc.content_sections {
            let section_type = section.section_type();
            if known_sections.contains(&section_type) {
                issues.push(Issue::DuplicateSection {
                    file: doc.path.clone(),
                    section_type: section_type.into(),
                });
            } else {
                known_sections.push(section_type);
            }
        }
    }
    issues
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use super::scan;
    use crate::testhelpers::{create_file, empty_config, tmp_dir};
    use crate::{Issue, Tikibase};

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
        let base = Tikibase::load(dir, &empty_config()).unwrap();
        let have = scan(&base);
        assert_eq!(
            have,
            vec![Issue::DuplicateSection {
                file: PathBuf::from("test.md"),
                section_type: "One".into(),
            }]
        )
    }
}
