use crate::core::tikibase::Tikibase;

/// finds all duplicate sections in the given Tikibase
pub fn process(base: &mut Tikibase) -> Vec<String> {
    let mut results = vec![];
    for doc in &mut base.docs {
        let mut known_sections = vec![];
        for section in &doc.content_sections {
            let section_type = section.section_type();
            if known_sections.contains(&section_type) {
                let filename = &doc.path.strip_prefix(&base.dir).unwrap().to_str().unwrap();
                results.push(format!(
                    "{}  duplicate section: {}",
                    &filename, &section_type
                ));
            } else {
                known_sections.push(section_type);
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {

    use super::process;
    use crate::core::tikibase::helpers;
    use std::path::PathBuf;

    #[test]
    fn duplicate_sections() {
        let content = "\
# test document

### One
content
### One
content";
        let mut base = helpers::testbase();
        base.create_doc(&PathBuf::from("test.md"), content);
        let have = process(&mut base);
        assert_eq!(have.len(), 1);
        assert_eq!(have[0], "test.md  duplicate section: One");
    }
}
