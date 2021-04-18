use crate::core::tikibase::Tikibase;

/// finds all empty sections in the given Tikibase,
/// fixes them if fix is enabled,
/// returns the unfixed issues
pub fn process(base: &mut Tikibase, fix: bool) -> Vec<String> {
    let mut results = vec![];
    for doc in &mut base.docs {
        let filename = &doc.path.strip_prefix(&base.dir).unwrap().to_str().unwrap();
        let mut fixed = false;
        doc.content_sections.retain(|section| {
            let has_content = section.body.iter().any(|line| !line.text.is_empty());
            if has_content {
                return true;
            }
            // found an empty section
            if fix {
                fixed = true;
                return false;
            }
            results.push(format!(
                "{}:{}  section \"{}\" has no content",
                &filename,
                section.line_number + 1,
                section.section_type()
            ));
            true
        });
        if fixed {
            crate::core::document::save(&doc.path, doc.text());
        }
    }
    results
}

#[cfg(test)]
mod tests {

    use super::process;
    use crate::core::document::Document;
    use crate::core::tikibase::helpers;
    use crate::core::tikibase::Tikibase;
    use std::path::PathBuf;

    #[test]
    fn false_empty_section() {
        let content = "\
# test document

### empty section
### next section

content";
        let mut base = helpers::testbase();
        base.create_doc(&PathBuf::from("test.md"), content);
        let have = process(&mut base, false);
        assert_eq!(have.len(), 1);
        assert_eq!(
            have[0],
            "test.md:3  section \"empty section\" has no content"
        );
    }

    #[test]
    fn false_empty_line() {
        let content = "\
# test document

### empty section

### next section

content";
        let doc = Document::from_str(content, PathBuf::from("test.md"));
        let mut base = Tikibase::with_doc(doc);
        let have = process(&mut base, false);
        assert_eq!(have.len(), 1);
        assert_eq!(
            have[0],
            "test.md:3  section \"empty section\" has no content"
        );
    }

    #[test]
    fn false_content() {
        let content = "\
# test document

### section with content

content";
        let doc = Document::from_str(content, PathBuf::from("test.md"));
        let mut base = Tikibase::with_doc(doc);
        let have = process(&mut base, false);
        assert_eq!(have.len(), 0);
    }
    #[test]
    fn true_empty_section() {
        let mut base = helpers::testbase();
        base.create_doc(
            &PathBuf::from("test.md"),
            "\
# test document

### empty section
### next section

content",
        );
        let result = process(&mut base, true);
        assert_eq!(result.len(), 0);
        let new_content = helpers::read_doc(&base, "test.md");
        assert_eq!(
            new_content,
            "\
# test document

### next section

content
"
        )
    }
}
