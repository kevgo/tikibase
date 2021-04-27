use super::result::Result;
use crate::core::tikibase::Tikibase;

/// finds all duplicate sections in the given Tikibase
pub fn process(base: &mut Tikibase) -> Result {
    let mut result = Result::new();
    for doc in &mut base.docs {
        let mut known_sections = vec![];
        for section in &doc.content_sections {
            let section_type = section.section_type();
            if known_sections.contains(&section_type) {
                let filename = &doc.path.to_string_lossy();
                result.findings.push(format!(
                    "{}  duplicate section: {}",
                    &filename, &section_type
                ));
            } else {
                known_sections.push(section_type);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::process;
    use crate::core::tikibase::Tikibase;
    use std::path::PathBuf;

    #[test]
    fn duplicate_sections() {
        let content = "\
# test document

### One
content
### One
content";
        let mut base = Tikibase::tmpbase();
        base.create_doc(PathBuf::from("test.md"), content);
        let have = process(&mut base);
        assert_eq!(have.findings.len(), 1);
        assert_eq!(have.findings[0], "test.md  duplicate section: One");
        assert_eq!(have.fixes.len(), 0);
    }
}
