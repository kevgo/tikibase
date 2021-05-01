use super::outcome::Outcome;
use crate::core::tikibase::Tikibase;

/// finds all empty sections in the given Tikibase,
/// fixes them if fix is enabled,
/// returns the unfixed issues
pub fn process(base: &mut Tikibase, fix: bool) -> Outcome {
    let mut result = Outcome::new();
    for doc in &mut base.docs {
        let filename = &doc.path.to_string_lossy();
        let mut fixed = false;
        doc.content_sections.retain(|section| {
            let has_content = section.body.iter().any(|line| !line.text.is_empty());
            if has_content {
                return true;
            }
            // found an empty section
            if fix {
                result.fixes.push(format!(
                    "{}:{}  removed empty section \"{}\"",
                    &filename,
                    section.line_number + 1,
                    section.section_type()
                ));
                fixed = true;
                return false;
            }
            result.findings.push(format!(
                "{}:{}  section \"{}\" has no content",
                &filename,
                section.line_number + 1,
                section.section_type()
            ));
            true
        });
        if fixed {
            doc.flush(&base.dir);
        }
    }
    result
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
