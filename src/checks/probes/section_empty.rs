use crate::checks::{Issue, Issues};
use crate::config;
use crate::database::Tikibase;
use std::path::PathBuf;

/// finds all empty sections in the given Tikibase,
/// fixes them if fix is enabled,
/// returns the unfixed issues
pub fn process(base: &Tikibase) -> Issues {
    let mut issues = Issues::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            let has_content = section.body.iter().any(|line| !line.text.is_empty());
            if !has_content {
                issues.push(Box::new(EmptySection {
                    filename: doc.path.clone(),
                    line: section.line_number,
                    section_type: section.section_type().into(),
                }));
            }
        }
    }
    issues
}

/// describes the issue that a section is empty
pub struct EmptySection {
    filename: PathBuf,
    line: u32,
    section_type: String,
}

impl Issue for EmptySection {
    fn fixable(&self) -> bool {
        true
    }

    fn fix(&self, base: &mut Tikibase, _config: &config::Data) -> String {
        let base_dir = &base.dir.clone();
        let doc = base.get_doc_mut(&self.filename).unwrap();
        doc.content_sections
            .retain(|section| section.section_type() != self.section_type);
        doc.flush(base_dir.as_ref());
        format!(
            "{}:{}  removed empty section \"{}\"",
            self.filename.to_string_lossy(),
            self.line + 1,
            self.section_type
        )
    }

    fn describe(&self) -> String {
        format!(
            "{}:{}  section \"{}\" has no content",
            self.filename.to_string_lossy(),
            self.line + 1,
            self.section_type
        )
    }
}

#[cfg(test)]
mod tests {

    use super::process;
    use crate::database::Tikibase;
    use crate::testhelpers::{create_file, empty_config, tmp_dir};

    #[test]
    fn empty_section() {
        let dir = tmp_dir();
        let content = "\
# test document

### empty section
### next section

content";
        create_file("test.md", content, &dir);
        let (base, errs) = Tikibase::load(dir, &empty_config());
        assert_eq!(errs.len(), 0);
        let have: Vec<String> = process(&base)
            .iter()
            .map(|issue| issue.describe())
            .collect();
        assert_eq!(have.len(), 1);
        assert_eq!(
            have[0],
            "test.md:3  section \"empty section\" has no content"
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
        let (base, errs) = Tikibase::load(dir, &empty_config());
        assert_eq!(errs.len(), 0);
        let have: Vec<String> = process(&base)
            .iter()
            .map(|issue| issue.describe())
            .collect();
        assert_eq!(have.len(), 1);
        assert_eq!(
            have[0],
            "test.md:3  section \"empty section\" has no content"
        );
    }

    #[test]
    fn content() {
        let dir = tmp_dir();
        let content = "\
# test document

### section with content

content";
        create_file("test.md", content, &dir);
        let (base, errs) = Tikibase::load(dir, &empty_config());
        assert_eq!(errs.len(), 0);
        let have = process(&base);
        assert!(have.is_empty());
    }
}
