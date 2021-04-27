use super::result::Result;
use super::Tikibase;
use crate::core::line::Reference;
use std::path::PathBuf;

pub fn process(base: &Tikibase) -> Result {
    let mut result = Result::new();
    let existing_targets = base.link_targets();
    for doc in &base.docs {
        for section in doc.sections() {
            for line in section.lines() {
                for reference in line.references() {
                    match reference {
                        Reference::Link { destination } => {
                            if !destination.starts_with("http")
                                && !existing_targets.contains(&destination)
                            {
                                result.findings.push(format!(
                                    "{}:{}  broken link to \"{}\"",
                                    &doc.path.to_string_lossy(),
                                    section.line_number + line.section_offset + 1,
                                    destination,
                                ));
                            }
                        }
                        Reference::Image { src } => {
                            if !src.starts_with("http") && !base.has_resource(PathBuf::from(&src)) {
                                result.findings.push(format!(
                                    "{}:{}  broken image \"{}\"",
                                    &doc.path.to_string_lossy(),
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

    mod process {
        use crate::core::tikibase::Tikibase;
        use std::path::PathBuf;

        #[test]
        fn link_to_non_existing_file() {
            let mut base = Tikibase::tmp();
            let content = "\
# One

[invalid](non-existing.md)
[valid](two.md)
";
            base.create_doc(PathBuf::from("one.md"), content);
            base.create_doc(PathBuf::from("two.md"), "# Two");
            let have = super::super::process(&base);
            let want = vec!["one.md:3  broken link to \"non-existing.md\""];
            assert_eq!(have.findings, want);
        }

        #[test]
        fn ignore_external_urls() {
            let mut base = Tikibase::tmp();
            let content = "\
# One

[external site](https://google.com)
![external image](https://google.com/foo.png)
";
            base.create_doc(PathBuf::from("one.md"), content);
            base.create_doc(PathBuf::from("two.md"), "# Two");
            let have = super::super::process(&base);
            let want: Vec<&str> = vec![];
            assert_eq!(have.findings, want);
        }
    }
}
