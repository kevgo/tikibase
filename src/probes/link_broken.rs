use super::outcome::Outcome;
use super::Tikibase;
use crate::core::line::Reference;
use std::path::PathBuf;

pub struct LinksResult {
    pub outcome: Outcome,
    /// all resources that are linked to from the given Tikibase
    pub resource_links: Vec<String>,
    /// all links between internal documents
    pub doc_links: DocLinks,
}

pub struct DocLink {
    pub from: PathBuf,
    pub to: PathBuf,
}

pub type DocLinks = Vec<DocLink>;

pub fn process(base: &Tikibase) -> LinksResult {
    let mut result = LinksResult {
        outcome: Outcome::new(),
        resource_links: Vec::new(),
        doc_links: Vec::new(),
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
                                result.doc_links.push(DocLink {
                                    from: doc.path.clone(),
                                    to: PathBuf::from(destination),
                                });
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
        use std::path::PathBuf;

        use crate::core::tikibase::Tikibase;
        use crate::testhelpers;

        #[test]
        fn link_to_non_existing_file() {
            let dir = testhelpers::tmp_dir();
            testhelpers::create_file("one.md", "# One\n\n[invalid](non-existing.md)\n", &dir);
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
        fn link_to_existing_file() {
            let dir = testhelpers::tmp_dir();
            let content = "\
# One

Here is a link to [Two](2.md) that works.

### section

Here is a link to [Three](3.md) that also works.

### another section

Here is a link to [Four](4.md) that also works.
";
            testhelpers::create_file("1.md", content, &dir);
            testhelpers::create_file("2.md", "# Two", &dir);
            testhelpers::create_file("3.md", "# Three", &dir);
            testhelpers::create_file("4.md", "# Four", &dir);
            let (base, errs) = Tikibase::load(dir);
            assert_eq!(errs.len(), 0);
            let have = super::super::process(&base);
            assert_eq!(have.outcome.findings.len(), 0);
            assert_eq!(have.doc_links.len(), 3);
            assert_eq!(have.doc_links[0].from, PathBuf::from("1.md"));
            assert_eq!(have.doc_links[0].to, PathBuf::from("2.md"));
            assert_eq!(have.doc_links[1].from, PathBuf::from("1.md"));
            assert_eq!(have.doc_links[1].to, PathBuf::from("3.md"));
            assert_eq!(have.doc_links[2].from, PathBuf::from("1.md"));
            assert_eq!(have.doc_links[2].to, PathBuf::from("4.md"));
        }

        #[test]
        fn external_urls() {
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
            testhelpers::create_file("1.md", "# One\n\n![image](foo.png)\n", &dir);
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
            testhelpers::create_file("1.md", "# One\n\n![image](zonk.png)\n", &dir);
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
