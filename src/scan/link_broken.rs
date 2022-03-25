use crate::database::{DocLinks, Reference, Tikibase};
use crate::{Issue, Location};

pub(crate) struct LinksResult {
    pub issues: Vec<Issue>,

    /// all links to documents
    pub incoming_doc_links: DocLinks,

    /// all links from documents
    pub outgoing_doc_links: DocLinks,

    /// all resources that are linked to from the given Tikibase
    pub outgoing_resource_links: Vec<String>,
}

pub(crate) fn scan(base: &Tikibase) -> LinksResult {
    let mut result = LinksResult {
        issues: Vec::new(),
        incoming_doc_links: DocLinks::default(),
        outgoing_doc_links: DocLinks::default(),
        outgoing_resource_links: Vec::default(),
    };
    let existing_targets = base.link_targets();
    for doc in &base.docs {
        let references = doc.references();
        if references.is_empty() {
            result.issues.push(Issue::DocumentWithoutLinks {
                location: Location {
                    file: doc.path.clone(),
                    line: 0,
                    start: 0,
                    end: 0,
                },
            });
            continue;
        }
        for reference in references {
            match reference {
                Reference::Link {
                    destination,
                    line,
                    start,
                    end,
                } => {
                    if destination.is_empty() {
                        result.issues.push(Issue::LinkWithoutDestination {
                            location: Location {
                                file: doc.path.clone(),
                                line,
                                start,
                                end,
                            },
                        });
                        continue;
                    }
                    if destination.starts_with("http") {
                        // ignore external links
                        continue;
                    }
                    if destination == doc.path.to_string_lossy() {
                        result.issues.push(Issue::LinkToSameDocument {
                            location: Location {
                                file: doc.path.clone(),
                                line,
                                start,
                                end,
                            },
                        });
                        continue;
                    }
                    // HACK: see https://github.com/rust-lang/rust/issues/42671#issuecomment-308713035
                    if !existing_targets
                        .iter()
                        .any(|existing_target| existing_target == link_anchor(&destination))
                    {
                        result.issues.push(Issue::BrokenLink {
                            location: Location {
                                file: doc.path.clone(),
                                line,
                                start,
                                end,
                            },
                            target: destination,
                        });
                        continue;
                    }
                    result
                        .incoming_doc_links
                        .add(&destination, doc.path.clone());
                    result.outgoing_doc_links.add(doc.path.clone(), destination);
                }
                Reference::Image {
                    src,
                    line,
                    start,
                    end,
                } => {
                    if src.starts_with("http") {
                        continue;
                    }
                    if !base.has_resource(&src) {
                        result.issues.push(Issue::BrokenImage {
                            location: Location {
                                file: doc.path.clone(),
                                line,
                                start,
                                end,
                            },
                            target: src.clone(),
                        });
                    }
                    result.outgoing_resource_links.push(src);
                }
            }
        }
    }
    result
}

/// converts the given URL into the anchor portion of it
fn link_anchor(link: &str) -> &str {
    // NOTE: it would probably be cleaner to return a &str to the portion of the given &String,
    // but that isn't needed here and it yields to type incompatibilities.
    // We are therefore reducing the string in place.
    if let Some(index) = link.find('#') {
        &link[index..]
    } else {
        link
    }
}

#[cfg(test)]
mod tests {

    mod link_anchor {
        use super::super::link_anchor;

        #[test]
        fn with_anchor() {
            let give = "1.md#foo".to_string();
            let want = "#foo".to_string();
            let have = link_anchor(&give);
            assert_eq!(have, want);
        }
    }

    mod process {
        use super::super::scan;
        use crate::{test, Config, Issue, Location, Tikibase};
        use indoc::indoc;
        use std::path::PathBuf;

        #[test]
        fn link_to_non_existing_file() {
            let dir = test::tmp_dir();
            test::create_file("one.md", "# One\n\n[invalid](non-existing.md)\n", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            let want = vec![Issue::BrokenLink {
                location: Location {
                    file: "one.md".into(),
                    line: 2,
                    start: 0,
                    end: 26,
                },
                target: "non-existing.md".into(),
            }];
            pretty::assert_eq!(have.issues, want);
            assert_eq!(have.incoming_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_resource_links.len(), 0);
        }

        #[test]
        fn link_to_existing_file() {
            let dir = test::tmp_dir();
            let content = indoc! {"
                # One

                Here is a link to [Two](2.md) that works.

                ### section

                Here is a link to [Three](3.md) that also works.
                "};
            test::create_file("1.md", content, &dir);
            test::create_file("2.md", "# Two\n[1](1.md)", &dir);
            test::create_file("3.md", "# Three\n[1](1.md)", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            assert_eq!(have.issues.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 3);
            let out_one = have.outgoing_doc_links.get("1.md").unwrap();
            assert_eq!(out_one.len(), 2);
            assert!(out_one.contains(&PathBuf::from("2.md")));
            assert!(out_one.contains(&PathBuf::from("3.md")));

            assert_eq!(have.incoming_doc_links.data.len(), 3);
            let into_two = have.incoming_doc_links.get("2.md").unwrap();
            assert_eq!(into_two.len(), 1);
            assert!(into_two.contains(&PathBuf::from("1.md")));
            let into_three = have.incoming_doc_links.get("3.md").unwrap();
            assert_eq!(into_three.len(), 1);
            assert!(into_three.contains(&PathBuf::from("1.md")));
        }

        #[test]
        fn link_without_destination() {
            let dir = test::tmp_dir();
            test::create_file("one.md", "# One\n\n[invalid]()\n", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            pretty::assert_eq!(
                have.issues,
                vec![Issue::LinkWithoutDestination {
                    location: Location {
                        file: "one.md".into(),
                        line: 2,
                        start: 0,
                        end: 11,
                    }
                }]
            );
            assert_eq!(have.incoming_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_resource_links.len(), 0);
        }

        #[test]
        fn external_urls() {
            let dir = test::tmp_dir();
            let content = indoc! {"
                # One

                [external site](https://google.com)
                ![external image](https://google.com/foo.png)
                "};
            test::create_file("one.md", content, &dir);
            test::create_file("two.md", "# Two\n[one](one.md)", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            assert!(have.issues.is_empty());
            assert_eq!(have.incoming_doc_links.data.len(), 1);
            assert_eq!(have.outgoing_doc_links.data.len(), 1);
            assert_eq!(have.outgoing_resource_links.len(), 0);
        }

        #[test]
        fn link_to_existing_image() {
            let dir = test::tmp_dir();
            test::create_file("1.md", "# One\n\n![image](foo.png)\n", &dir);
            test::create_file("foo.png", "image content", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            assert!(have.issues.is_empty());
            assert_eq!(have.outgoing_resource_links.len(), 1);
            assert_eq!(have.outgoing_resource_links[0], "foo.png");
            assert_eq!(have.incoming_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 0);
        }

        #[test]
        fn link_to_non_existing_image() {
            let dir = test::tmp_dir();
            test::create_file("1.md", "# One\n\n![image](zonk.png)\n", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            let want = vec![Issue::BrokenImage {
                location: Location {
                    file: "1.md".into(),
                    line: 2,
                    start: 0,
                    end: 18,
                },
                target: "zonk.png".into(),
            }];
            pretty::assert_eq!(have.issues, want);
            assert_eq!(have.outgoing_resource_links.len(), 1);
            assert_eq!(have.outgoing_resource_links[0], "zonk.png");
            assert_eq!(have.incoming_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 0);
        }
    }
}
