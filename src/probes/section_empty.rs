use std::path::PathBuf;

use super::outcome::Issue;
use crate::core::tikibase::Tikibase;

/// finds all empty sections in the given Tikibase,
/// fixes them if fix is enabled,
/// returns the unfixed issues
pub fn process(base: &Tikibase) -> Vec<Box<dyn Issue>> {
    let mut result = Vec::<Box<dyn Issue>>::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            let has_content = section.body.iter().any(|line| !line.text.is_empty());
            if !has_content {
                result.push(Box::new(EmptySection {
                    filename: doc.path.clone(),
                    line: section.line_number,
                    section_type: section.section_type(),
                }));
            }
        }
    }
    result
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

    fn fix(self, base: &mut Tikibase) -> String {
        let base_dir = &base.dir.clone();
        let doc = base.get_doc_mut(&self.filename).unwrap();
        doc.content_sections
            .retain(|section| section.section_type() == self.section_type);
        doc.flush(base_dir.as_ref());
        format!(
            "{}:{}  removed empty section \"{}\"",
            self.filename.to_string_lossy(),
            self.line + 1,
            self.section_type
        )
    }

    fn describe(self) -> String {
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
    use crate::core::tikibase::Tikibase;
    use crate::testhelpers;

    #[test]
    fn false_empty_section() {
        let dir = testhelpers::tmp_dir();
        let content = "\
# test document

### empty section
### next section

content";
        testhelpers::create_file("test.md", content, &dir);
        let (mut base, errs) = Tikibase::load(dir);
        assert_eq!(errs.len(), 0);
        let have = process(&mut base, false);
        assert_eq!(have.findings.len(), 1);
        assert_eq!(
            have.findings[0],
            "test.md:3  section \"empty section\" has no content"
        );
    }

    #[test]
    fn false_empty_line() {
        let dir = testhelpers::tmp_dir();
        let content = "\
# test document

### empty section

### next section

content";
        testhelpers::create_file("test.md", content, &dir);
        let (mut base, errs) = Tikibase::load(dir);
        assert_eq!(errs.len(), 0);
        let have = process(&mut base, false);
        assert_eq!(have.findings.len(), 1);
        assert_eq!(
            have.findings[0],
            "test.md:3  section \"empty section\" has no content"
        );
    }

    #[test]
    fn false_content() {
        let dir = testhelpers::tmp_dir();
        let content = "\
# test document

### section with content

content";
        testhelpers::create_file("test.md", content, &dir);
        let (mut base, errs) = Tikibase::load(dir);
        assert_eq!(errs.len(), 0);
        let have = process(&mut base, false);
        assert_eq!(have.findings.len(), 0);
    }

    #[test]
    fn true_empty_section() {
        let dir = testhelpers::tmp_dir();
        testhelpers::create_file(
            "test.md",
            "\
# test document

### empty section
### next section

content",
            &dir,
        );
        let (mut base, errs) = Tikibase::load(dir);
        assert_eq!(errs.len(), 0);
        let result = process(&mut base, true);
        assert_eq!(result.findings.len(), 0);

        // verify Tikibase data
        assert_eq!(base.docs.len(), 1);
        assert_eq!(base.docs[0].content_sections.len(), 1);
        assert_eq!(
            base.docs[0].content_sections[0].title_line.text,
            "### next section"
        );
    }
}
