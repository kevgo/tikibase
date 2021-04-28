use super::outcome::Outcome;
use crate::core::tikibase::Tikibase;

/// finds all duplicate sections in the given Tikibase
pub fn process(base: &mut Tikibase) -> Outcome {
    let mut result = Outcome::new();
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
    use crate::core::{error::UserError, tikibase::Tikibase};
    use crate::testhelpers;

    #[test]
    fn duplicate_sections() -> Result<(), UserError> {
        let dir = testhelpers::tmp_dir();
        let content = "\
# test document

### One
content
### One
content";
        testhelpers::create_file("test.md", content, &dir);
        let mut base = Tikibase::load(dir)?;
        let have = process(&mut base);
        assert_eq!(have.findings.len(), 1);
        assert_eq!(have.findings[0], "test.md  duplicate section: One");
        assert_eq!(have.fixes.len(), 0);
        Ok(())
    }
}
