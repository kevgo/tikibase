use super::checker::Location;
use crate::core::tikibase::Tikibase;

use super::checker::LocalizedIssue;

pub struct EmptySection {
    location: Location,
}

impl LocalizedIssue for EmptySection {
    fn desc(&self) -> String {
        "empty section".to_string()
    }
    fn location(&self) -> String {
        self.location.to_string()
    }
}

/// provides all empty sections in the given Tikibase
pub fn find(base: &Tikibase) -> Vec<Box<dyn LocalizedIssue>> {
    let mut result: Vec<Box<dyn LocalizedIssue>> = Vec::new();
    for doc in &base.docs {
        for section in &doc.content_sections {
            let has_content = section.body.iter().any(|line| !line.text.is_empty());
            if !has_content {
                result.push(Box::new(EmptySection {
                    location: Location::from_path(&doc.path, section.line_number),
                }));
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
            assert_eq!(have[0].location(), "test.md:2");
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
            assert_eq!(have[0].location(), "test.md:2");
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
