use super::outcome::Outcome;
use super::Tikibase;
use crate::core::line::Reference;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct LinksResult<'a> {
    pub outcome: Outcome,
    /// all resources that are linked to from the given Tikibase
    pub resource_links: Vec<String>,
    /// all internal links from source file --> destination document
    pub doc_links: HashMap<&'a PathBuf, PathBuf>,
}

pub fn process(base: &Tikibase) -> LinksResult {
    let mut result = LinksResult {
        outcome: Outcome::new(),
        resource_links: Vec::new(),
        doc_links: HashMap::new(),
    };
    let existing_targets = base.link_targets();
    for doc in &base.docs {
        for section in doc.sections() {
            for line in section.lines() {
                for reference in line.references() {
                    match reference {
                        Reference::Link { mut destination } => {
                            if destination.starts_with("http") {
                                continue;
                            }
                            if let Some(index) = destination.find('#') {
                                destination.replace_range(..index, "");
                            }
                            if !existing_targets.contains(&destination) {
                                result.outcome.findings.push(format!(
                                    "{}:{}  broken link to \"{}\"",
                                    &doc.path.to_string_lossy(),
                                    section.line_number + line.section_offset + 1,
                                    destination,
                                ));
                            } else {
                                result
                                    .doc_links
                                    .insert(&doc.path, PathBuf::from(destination));
                            }
                        }
                        Reference::Image { src } => {
                            if src.starts_with("http") {
                                continue;
                            }
                            if !base.has_resource(PathBuf::from(&src)) {
                                result.outcome.findings.push(format!(
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
        use crate::testhelpers;

        #[test]
        fn link_to_non_existing_file() {
            let dir = testhelpers::tmp_dir();
            let content = "\
# One

[invalid](non-existing.md)
[valid](two.md)
";
            testhelpers::create_file("one.md", content, &dir);
            testhelpers::create_file("two.md", "# Two", &dir);
            let (base, errs) = Tikibase::load(dir);
            assert_eq!(errs.len(), 0);
            let have = super::super::process(&base);
            assert_eq!(
                have.outcome.findings,
                vec!["one.md:3  broken link to \"non-existing.md\""]
            );
            assert_eq!(have.doc_links.len(), 0);
        }

        #[test]
        fn ignore_external_urls() {
            let dir = testhelpers::tmp_dir();
            let content = "\
# One

[external site](https://google.com)
![external image](https://google.com/foo.png)
";
            testhelpers::create_file("one.md", content, &dir);
            testhelpers::create_file("two.md", "# Two", &dir);
            let (base, errs) = Tikibase::load(dir);
            assert_eq!(errs.len(), 0);
            let have = super::super::process(&base);
            let want: Vec<&str> = vec![];
            assert_eq!(have.outcome.findings, want);
            assert_eq!(have.doc_links.len(), 0);
        }

        #[test]
        fn link_to_existing_image() {
            let dir = testhelpers::tmp_dir();
            let content = "\
# One

![image](foo.png)
";
            testhelpers::create_file("1.md", content, &dir);
            testhelpers::create_file("foo.png", "image content", &dir);
            let (base, errs) = Tikibase::load(dir);
            assert_eq!(errs.len(), 0);
            let have = super::super::process(&base);
            assert_eq!(have.outcome.findings.len(), 0);
            assert_eq!(have.resource_links.len(), 1);
            assert_eq!(have.resource_links[0], "foo.png");
            assert_eq!(have.doc_links.len(), 0);
        }

        #[test]
        fn link_to_non_existing_image() {
            let dir = testhelpers::tmp_dir();
            let content = "\
# One

![image](zonk.png)
";
            testhelpers::create_file("1.md", content, &dir);
            let (base, errs) = Tikibase::load(dir);
            assert_eq!(errs.len(), 0);
            let have = super::super::process(&base);
            assert_eq!(have.outcome.findings.len(), 1);
            assert_eq!(
                have.outcome.findings[0],
                "1.md:3  broken image \"zonk.png\""
            );
            assert_eq!(have.resource_links.len(), 1);
            assert_eq!(have.resource_links[0], "zonk.png");
            assert_eq!(have.doc_links.len(), 0);
        }
    }
}
