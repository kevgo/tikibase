use super::document::Document;
use super::persistence;
use super::resource::Resource;
use heck::KebabCase;
use std::path::{Path, PathBuf};

pub struct Tikibase {
    pub dir: PathBuf,
    pub docs: Vec<Document>,
    pub resources: Vec<Resource>,
}

impl Tikibase {
    /// creates a new document with the given content in this Tikibase
    pub fn create_doc(&mut self, filename: &Path, content: &str) {
        let filepath = self.dir.join(filename);
        persistence::save_file(&filepath, content);
        self.docs.push(Document::from_str(filepath, content));
    }

    /// provides all valid link targets in this Tikibase
    pub fn link_targets(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for doc in &self.docs {
            let filename = doc.relative_path(&self.dir);
            result.push(format!(
                "{}{}",
                &filename,
                make_anchor(&doc.title_section.title_line)
            ));
            for section in &doc.content_sections {
                result.push(format!("{}{}", &filename, make_anchor(&section.title_line)));
            }
            result.push(filename);
        }
        result.sort();
        result
    }
}

/// converts the given link title into a GitHub-compatible link anchor
fn make_anchor(title: &str) -> String {
    format!("#{}", title.to_kebab_case())
}

#[cfg(test)]
mod tests {
    use crate::core::persistence;
    use std::path::PathBuf;

    #[test]
    fn make_anchor() {
        let tests = vec![
            ("foo", "#foo"),
            ("what is it", "#what-is-it"),
            ("A Complex Section", "#a-complex-section"),
        ];
        for (give, want) in tests.into_iter() {
            assert_eq!(super::make_anchor(give), want);
        }
    }

    #[test]
    fn link_targets() {
        let mut base = persistence::tmpbase();
        let content = "\
# One

### Alpha
### Beta

content";
        base.create_doc(&PathBuf::from("one.md"), content);
        base.create_doc(&PathBuf::from("two.md"), content);
        let have = base.link_targets();
        let want = vec![
            "one.md",
            "one.md#alpha",
            "one.md#beta",
            "one.md#one",
            "two.md",
            "two.md#alpha",
            "two.md#beta",
            "two.md#one",
        ];
        assert_eq!(have, want);
    }
}
