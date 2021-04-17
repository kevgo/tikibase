use crate::core::tikibase::Tikibase;

/// finds all empty sections in the given Tikibase,
/// fixes them if fix is enabled,
/// returns the unfixed issues
pub fn process(base: &mut Tikibase, fix: bool) -> Vec<String> {
    let mut results = vec![];
    for doc in &base.docs {
        for section in &doc.content_sections {
            let has_content = section.body.iter().any(|line| !line.text.is_empty());
            if !has_content {
                // found an empty section
                if fix {
                    // TODO
                } else {
                    results.push(format!(
                        "{}:{}  section \"{}\" has no content",
                        &doc.path.strip_prefix(&base.dir).unwrap().to_str().unwrap(),
                        section.line_number + 1,
                        section.section_type()
                    ));
                }
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod find {

        use crate::core::tikibase::Tikibase;

        #[test]
        fn section_without_body_lines() {
            let content = "\
# test document

### empty section
### next section

content";
            let doc = crate::core::document::Document::from_str(content, "test.md");
            let mut base = Tikibase::with_doc(doc);
            let have = super::super::process(&mut base, false);
            assert_eq!(have.len(), 1);
            assert_eq!(
                have[0],
                "test.md:3  section \"empty section\" has no content"
            );
        }

        #[test]
        fn section_with_empty_body_line() {
            let content = "\
# test document

### empty section

### next section

content";
            let doc = crate::core::document::Document::from_str(content, "test.md");
            let mut base = Tikibase::with_doc(doc);
            let have = super::super::process(&mut base, false);
            assert_eq!(have.len(), 1);
            assert_eq!(
                have[0],
                "test.md:3  section \"empty section\" has no content"
            );
        }

        #[test]
        fn section_with_content() {
            let content = "\
# test document

### section with content

content";
            let doc = crate::core::document::Document::from_str(content, "test.md");
            let mut base = Tikibase::with_doc(doc);
            let have = super::super::process(&mut base, false);
            assert_eq!(have.len(), 0);
        }
    }
}
