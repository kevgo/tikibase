use crate::core::tikibase::Tikibase;

/// provides all empty sections in the given Tikibase
pub fn find(base: &Tikibase) -> Vec<String> {
    let mut result = vec![];
    for doc in &base.docs {
        for section in &doc.content_sections {
            let has_content = section.body.iter().any(|line| !line.text.is_empty());
            if !has_content {
                result.push(format!(
                    "{}:{}  section \"{}\" has no content",
                    &doc.path.strip_prefix(&base.dir).unwrap().to_str().unwrap(),
                    section.line_number + 1,
                    section.section_type()
                ));
            }
        }
    }
    result
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
            let base = Tikibase::with_doc(doc);
            let have = super::super::find(&base);
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
            let base = Tikibase::with_doc(doc);
            let have = super::super::find(&base);
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
            let base = Tikibase::with_doc(doc);
            let have = super::super::find(&base);
            assert_eq!(have.len(), 0);
        }
    }
}
