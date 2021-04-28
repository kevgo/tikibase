use super::result::Result;
use super::Tikibase;
use crate::core::line::Reference;
use std::path::PathBuf;

pub struct LinksResult {
    pub result: Result,
    pub resource_links: Vec<String>,
}

pub fn process(base: &Tikibase) -> LinksResult {
    let mut result = LinksResult {
        result: Result::new(),
        resource_links: Vec::new(),
    };
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
                                result.result.findings.push(format!(
                                    "{}:{}  broken link to \"{}\"",
                                    &doc.path.to_string_lossy(),
                                    section.line_number + line.section_offset + 1,
                                    destination,
                                ));
                            }
                        }
                        Reference::Image { src } => {
                            if src.starts_with("http") {
                                continue;
                            }
                            if !base.has_resource(PathBuf::from(&src)) {
                                result.result.findings.push(format!(
                                    "{}:{}  broken image \"{}\"",
                                    &doc.path.to_string_lossy(),
                                    section.line_number + line.section_offset + 1,
                                    &src,
                                ));
                            }
                            result.resource_links.push(src);
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
            assert_eq!(have.result.findings, want);
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
            assert_eq!(have.result.findings, want);
        }

        #[test]
        fn link_to_existing_image() {
            let mut base = Tikibase::tmp();
            let content = "\
# One

![image](foo.png)
";
            base.create_doc(PathBuf::from("1.md"), content);
            base.create_resource(PathBuf::from("foo.png"), "image content");
            let have = super::super::process(&base);
            assert_eq!(have.result.findings.len(), 0);
            assert_eq!(have.resource_links.len(), 1);
            assert_eq!(have.resource_links[0], "foo.png")
        }

        #[test]
        fn link_to_non_existing_image() {
            let mut base = Tikibase::tmp();
            let content = "\
# One

![image](zonk.png)
";
            base.create_doc(PathBuf::from("1.md"), content);
            let have = super::super::process(&base);
            assert_eq!(have.result.findings.len(), 1);
            assert_eq!(have.result.findings[0], "1.md:3  broken image \"zonk.png\"");
            assert_eq!(have.resource_links.len(), 1);
            assert_eq!(have.resource_links[0], "zonk.png")
            //
        }
    }
}
