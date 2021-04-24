use super::result::Result;
use super::Tikibase;

pub fn process(base: &Tikibase) -> Result {
    let mut result = Result::new();
    let existing_targets = base.link_targets();
    for doc in &base.docs {
        println!("DOC: {}", doc.filename());
        for section in &doc.content_sections {
            for line in &section.body {
                for link in line.links() {
                    if !existing_targets.contains(&link.destination) {
                        result.findings.push(format!(
                            "{}:{}  broken link to {}",
                            doc.filename(),
                            section.line_number + line.section_offset,
                            link.destination,
                        ));
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::core::persistence;
    use std::path::PathBuf;

    #[test]
    fn process() {
        let mut base = persistence::tmpbase();
        let content = "\
# One

[broken](non-existing.md)
[valid](two.md)
";
        base.create_doc(&PathBuf::from("one.md"), content);
        base.create_doc(&PathBuf::from("two.md"), "# Two");
        let have = super::process(&base);
        let want = vec!["one.md:1  broken link to non-existing.md"];
        assert_eq!(have.findings, want);
    }
}
