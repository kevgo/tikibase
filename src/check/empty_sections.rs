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

        use crate::core::document::Document;
        use crate::core::line::Line;
        use crate::core::section::Section;
        use crate::core::tikibase::Tikibase;

        #[test]
        fn section_without_body_lines() {
            let title_section = Section {
                title_line: "# test document".to_string(),
                line_number: 0,
                body: vec![],
            };
            let section_with_empty_line = Section {
                title_line: "### empty section".to_string(),
                line_number: 12,
                body: vec![],
            };
            let doc = Document {
                path: std::path::PathBuf::from("test.md"),
                title_section: title_section,
                content_sections: vec![section_with_empty_line],
            };
            let base = Tikibase {
                dir: "".to_string(),
                docs: vec![doc],
                resources: vec![],
            };
            let have = super::super::find(&base);
            assert_eq!(have.len(), 1);
            assert_eq!(have[0].path.to_str().unwrap(), "test.md");
            assert_eq!(have[0].line, 12)
        }

        #[test]
        fn section_with_empty_body_line() {
            let title_section = Section {
                title_line: "# test document".to_string(),
                line_number: 0,
                body: vec![],
            };
            let empty_line = Line {
                text: "".to_string(),
                section_offset: 1,
            };
            let section_with_empty_line = Section {
                title_line: "### section with empty line".to_string(),
                line_number: 12,
                body: vec![empty_line],
            };
            let doc = Document {
                path: std::path::PathBuf::from("test.md"),
                title_section: title_section,
                content_sections: vec![section_with_empty_line],
            };
            let base = Tikibase {
                dir: "".to_string(),
                docs: vec![doc],
                resources: vec![],
            };
            let have = super::super::find(&base);
            assert_eq!(have.len(), 1);
            assert_eq!(have[0].path.to_str().unwrap(), "test.md");
            assert_eq!(have[0].line, 12)
        }

        #[test]
        fn section_with_content() {
            let title_section = Section {
                title_line: "# test document".to_string(),
                line_number: 0,
                body: vec![],
            };
            let line = Line {
                text: "some content".to_string(),
                section_offset: 1,
            };
            let section_with_content = Section {
                title_line: "### section with content".to_string(),
                line_number: 12,
                body: vec![line],
            };
            let doc = Document {
                path: std::path::PathBuf::from("test.md"),
                title_section: title_section,
                content_sections: vec![section_with_content],
            };
            let base = Tikibase {
                dir: "".to_string(),
                docs: vec![doc],
                resources: vec![],
            };
            let have = super::super::find(&base);
            assert_eq!(have.len(), 0);
        }
    }
}
