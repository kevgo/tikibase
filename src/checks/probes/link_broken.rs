use crate::checks::doc_links::DocLinks;
use crate::checks::{issues, Issue, Issues};
use crate::config;
use crate::database::Reference;
use crate::database::Tikibase;
use std::path::PathBuf;

pub struct LinksResult {
    pub issues: Issues,

    /// all links to documents
    pub incoming_doc_links: DocLinks,

    /// all links from documents
    pub outgoing_doc_links: DocLinks,

    /// all resources that are linked to from the given Tikibase
    pub outgoing_resource_links: Vec<String>,
}

pub fn process(base: &Tikibase) -> LinksResult {
    let mut result = LinksResult {
        issues: Issues::new(),
        incoming_doc_links: DocLinks::new(),
        outgoing_doc_links: DocLinks::new(),
        outgoing_resource_links: Vec::new(),
    };
    let existing_targets = base.link_targets();
    for doc in &base.docs {
        for section in doc.sections() {
            for (i, line) in section.lines().enumerate() {
                for reference in line.references() {
                    match reference {
                        Reference::Link { mut destination } => {
                            if destination.is_empty() {
                                result.issues.push(Box::new(LinkWithoutDestination {
                                    filename: doc.path.clone(),
                                    line: section.line_number + (i as u32) + 1,
                                }));
                                continue;
                            }
                            if destination.starts_with("http") {
                                continue;
                            }
                            if let Some(index) = destination.find('#') {
                                destination.replace_range(..index, "");
                            }
                            if !existing_targets.contains(&destination) {
                                result.issues.push(Box::new(issues::BrokenLink {
                                    filename: doc.path.clone(),
                                    line: section.line_number + (i as u32) + 1,
                                    target: destination,
                                }));
                                continue;
                            }
                            if destination == doc.path.to_string_lossy() {
                                result.issues.push(Box::new(LinkToSameDocument {
                                    filename: doc.path.clone(),
                                    line: section.line_number + (i as u32) + 1,
                                }));
                                continue;
                            }
                            result
                                .incoming_doc_links
                                .add(&destination, doc.path.clone());
                            result.outgoing_doc_links.add(doc.path.clone(), destination);
                        }
                        Reference::Image { src } => {
                            if src.starts_with("http") {
                                continue;
                            }
                            if !base.has_resource(&src) {
                                result.issues.push(Box::new(issues::BrokenImage {
                                    filename: doc.path.clone(),
                                    line: section.line_number + (i as u32) + 1,
                                    target: src.clone(),
                                }));
                            }
                            result.outgoing_resource_links.push(src);
                        }
                    }
                }
            }
        }
    }
    result
}

pub struct LinkToSameDocument {
    filename: PathBuf,
    line: u32,
}

impl Issue for LinkToSameDocument {
    fn describe(&self) -> String {
        format!(
            "{}:{}  link to the same file",
            self.filename.to_string_lossy(),
            self.line
        )
    }

    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        panic!("not fixable");
    }

    fn fixable(&self) -> bool {
        false
    }
}

pub struct LinkWithoutDestination {
    filename: PathBuf,
    line: u32,
}

impl Issue for LinkWithoutDestination {
    fn describe(&self) -> String {
        format!(
            "{}:{}  link without destination",
            self.filename.to_string_lossy(),
            self.line
        )
    }

    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        panic!("not fixable");
    }

    fn fixable(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {

    mod process {
        use std::path::PathBuf;

        use crate::database::Tikibase;
        use crate::testhelpers::{create_file, empty_config, tmp_dir};

        #[test]
        fn link_to_non_existing_file() {
            let dir = tmp_dir();
            create_file("one.md", "# One\n\n[invalid](non-existing.md)\n", &dir);
            let (base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
            let have = super::super::process(&base);
            let outcomes: Vec<String> = have.issues.iter().map(|issue| issue.describe()).collect();
            assert_eq!(
                outcomes,
                vec!["one.md:3  broken link to \"non-existing.md\""]
            );
            assert_eq!(have.incoming_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_resource_links.len(), 0);
        }

        #[test]
        fn link_to_existing_file() {
            let dir = tmp_dir();
            let content = "\
# One

Here is a link to [Two](2.md) that works.

### section

Here is a link to [Three](3.md) that also works.
";
            create_file("1.md", content, &dir);
            create_file("2.md", "# Two", &dir);
            create_file("3.md", "# Three", &dir);
            let (base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
            let have = super::super::process(&base);
            assert!(have.issues.is_empty());
            assert_eq!(have.outgoing_doc_links.data.len(), 1);
            let out_one = have.outgoing_doc_links.get("1.md").unwrap();
            assert_eq!(out_one.len(), 2);
            assert!(out_one.contains(&PathBuf::from("2.md")));
            assert!(out_one.contains(&PathBuf::from("3.md")));

            assert_eq!(have.incoming_doc_links.data.len(), 2);
            let into_two = have.incoming_doc_links.get("2.md").unwrap();
            assert_eq!(into_two.len(), 1);
            assert!(into_two.contains(&PathBuf::from("1.md")));
            let into_three = have.incoming_doc_links.get("3.md").unwrap();
            assert_eq!(into_three.len(), 1);
            assert!(into_three.contains(&PathBuf::from("1.md")));
        }

        #[test]
        fn link_without_destination() {
            let dir = tmp_dir();
            create_file("one.md", "# One\n\n[invalid]()\n", &dir);
            let (base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
            let have = super::super::process(&base);
            let outcomes: Vec<String> = have.issues.iter().map(|issue| issue.describe()).collect();
            assert_eq!(outcomes, vec!["one.md:3  link without destination"]);
            assert_eq!(have.incoming_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_resource_links.len(), 0);
        }

        #[test]
        fn external_urls() {
            let dir = tmp_dir();
            let content = "\
# One

[external site](https://google.com)
![external image](https://google.com/foo.png)
";
            create_file("one.md", content, &dir);
            create_file("two.md", "# Two", &dir);
            let (base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
            let have = super::super::process(&base);
            assert!(have.issues.is_empty());
            assert_eq!(have.incoming_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_resource_links.len(), 0);
        }

        #[test]
        fn link_to_existing_image() {
            let dir = tmp_dir();
            create_file("1.md", "# One\n\n![image](foo.png)\n", &dir);
            create_file("foo.png", "image content", &dir);
            let (base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
            let have = super::super::process(&base);
            assert!(have.issues.is_empty());
            assert_eq!(have.outgoing_resource_links.len(), 1);
            assert_eq!(have.outgoing_resource_links[0], "foo.png");
            assert_eq!(have.incoming_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 0);
        }

        #[test]
        fn link_to_non_existing_image() {
            let dir = tmp_dir();
            create_file("1.md", "# One\n\n![image](zonk.png)\n", &dir);
            let (base, errs) = Tikibase::load(dir, &empty_config());
            assert_eq!(errs.len(), 0);
            let have = super::super::process(&base);
            let outcomes: Vec<String> = have.issues.iter().map(|issue| issue.describe()).collect();
            assert_eq!(outcomes.len(), 1);
            assert_eq!(outcomes[0], "1.md:3  broken image \"zonk.png\"");
            assert_eq!(have.outgoing_resource_links.len(), 1);
            assert_eq!(have.outgoing_resource_links[0], "zonk.png");
            assert_eq!(have.incoming_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 0);
        }
    }
}
