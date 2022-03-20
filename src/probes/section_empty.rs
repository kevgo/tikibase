use crate::{Issue, Tikibase};

/// finds all empty sections in the given Tikibase,
/// fixes them if fix is enabled,
/// returns the unfixed issues
pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            let has_content = section.body.iter().any(|line| !line.text().is_empty());
            if !has_content {
                issues.push(Issue::EmptySection {
                    file: doc.path.clone(),
                    line: section.line_number,
                    section_type: section.section_type().into(),
                });
            }
        }
    }
    issues
}

#[cfg(test)]
mod tests {

    use super::scan;
    use crate::testhelpers::{create_file, empty_config, tmp_dir};
    use crate::{Issue, Tikibase};
    use std::path::PathBuf;

    #[test]
    fn empty_section() {
        let dir = tmp_dir();
        let content = "\
# test document

### empty section
### next section

content";
        create_file("test.md", content, &dir);
        let base = Tikibase::load(dir, &empty_config()).unwrap();
        let have = scan(&base);
        assert_eq!(
            have,
            vec![Issue::EmptySection {
                file: PathBuf::from("test.md"),
                line: 2,
                section_type: "empty section".into()
            }]
        );
    }

    #[test]
    fn blank_line() {
        let dir = tmp_dir();
        let content = "\
# test document

### empty section

### next section

content";
        create_file("test.md", content, &dir);
        let base = Tikibase::load(dir, &empty_config()).unwrap();
        let have = scan(&base);
        assert_eq!(have.len(), 1);
        assert_eq!(
            have,
            vec![Issue::EmptySection {
                file: PathBuf::from("test.md"),
                line: 2,
                section_type: "empty section".into()
            }]
        )
    }

    #[test]
    fn content() {
        let dir = tmp_dir();
        let content = "\
# test document

### section with content

content";
        create_file("test.md", content, &dir);
        let base = Tikibase::load(dir, &empty_config()).unwrap();
        let have = scan(&base);
        assert!(have.is_empty());
    }
}
