use super::result::Result;
use super::Tikibase;
use crate::core::document;
use crate::core::line::Reference;

pub fn process(base: &Tikibase) -> Result {
    let mut result = Result::new();
    let existing_targets = base.link_targets();
    for doc in &base.docs {
        for section in doc.sections() {
            for line in section.lines() {
                for link in line.references() {
                    match link {
                        Reference::Link { destination, title } => {
                            if !existing_targets.contains(&destination) {
                                result.findings.push(format!(
                                    "{}:{}  broken link \"{}\" to \"{}\"",
                                    document::relative_path(&doc.path, &base.dir),
                                    section.line_number + line.section_offset + 1,
                                    title,
                                    destination,
                                ));
                            }
                        }
                        Reference::Image { src } => {
                            if !existing_targets.contains(&src) {
                                result.findings.push(format!(
                                    "{}:{}  broken image \"{}\"",
                                    document::relative_path(&doc.path, &base.dir),
                                    section.line_number + line.section_offset + 1,
                                    src,
                                ));
                            }
                        }
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

[invalid](non-existing.md)
[valid](two.md)
";
        base.create_doc(&PathBuf::from("one.md"), content);
        base.create_doc(&PathBuf::from("two.md"), "# Two");
        let have = super::process(&base);
        let want = vec!["one.md:3  broken link \"invalid\" to \"non-existing.md\""];
        assert_eq!(have.findings, want);
    }
}
