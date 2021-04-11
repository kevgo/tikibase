use crate::core::tikibase::Tikibase;
use std::path::PathBuf;

pub struct EmptySection {
    pub path: PathBuf,
    pub line: u32,
}

/// provides all empty sections in the given Tikibase
pub fn find(base: &Tikibase) -> Vec<EmptySection> {
    let mut result = Vec::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            let has_content = section.body.iter().any(|line| !line.text.is_empty());
            if !has_content {
                result.push(EmptySection {
                    path: doc.path.clone(),
                    line: section.line_number,
                });
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
            assert_eq!(have[0].path.to_str().unwrap(), "test.md");
            assert_eq!(have[0].line, 2)
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
            assert_eq!(have[0].path.to_str().unwrap(), "test.md");
            assert_eq!(have[0].line, 2)
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
