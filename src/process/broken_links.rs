use super::result::Result;
use super::Tikibase;

pub fn process(base: &Tikibase) -> Result {
    let mut result = Result::new();
    let existing_targets = base.link_targets();
    for doc in &base.docs {
        for section in doc.sections() {
            for line in section.lines() {
                for link in line.links() {
                    if !existing_targets.contains(&link.destination) {
                        result.findings.push(format!(
                            "{}:{}  broken link to \"{}\"",
                            crate::core::document::relative_path(&doc.path, &base.dir),
                            section.line_number + line.section_offset + 1,
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
        let want = vec!["one.md:3  broken link to \"non-existing.md\""];
        assert_eq!(have.findings, want);
    }
}
