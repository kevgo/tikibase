use crate::database::{DocLinks, Reference, Tikibase};
use crate::{Issue, Location};

#[derive(Default)]
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
    let mut result = LinksResult::default();
    for doc in base.documents() {
        let references = doc.references();
        if references.is_empty() {
            result.issues.push(Issue::DocumentWithoutLinks {
                location: Location {
                    file: doc.relative_path.clone(),
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
                    target,
                    line,
                    start,
                    end,
                } => {
                    if target.is_empty() {
                        result.issues.push(Issue::LinkWithoutTarget {
                            location: Location {
                                file: doc.relative_path.clone(),
                                line,
                                start,
                                end,
                            },
                        });
                        continue;
                    }
                    if target.starts_with("http") {
                        // ignore external links
                        continue;
                    }
                    let (target_file, target_anchor) = match target.split_once('#') {
                        Some((base, anchor)) => (base.to_string(), anchor.to_string()),
                        None => (target.clone(), "".to_string()),
                    };
                    if target_file == doc.relative_path.to_string_lossy() {
                        result.issues.push(Issue::LinkToSameDocument {
                            location: Location {
                                file: doc.relative_path.clone(),
                                line,
                                start,
                                end,
                            },
                        });
                        continue;
                    }
                    if let Some(anchor_without_prefix) = target.strip_prefix('#') {
                        let full_target =
                            format!("{}{}", doc.relative_path.to_string_lossy(), target);
                        if !strings_contain(&existing_targets, &full_target) {
                            result
                                .issues
                                .push(Issue::LinkToNonExistingAnchorInCurrentDocument {
                                    location: Location {
                                        file: doc.relative_path.clone(),
                                        line,
                                        start,
                                        end,
                                    },
                                    anchor: anchor_without_prefix.into(),
                                });
                            continue;
                        }
                    }
                    match FileType::from(&target_file) {
                        FileType::Document => {
                            if !strings_contain(&existing_targets, &target) {
                                if strings_contain(&existing_targets, &target_file) {
                                    result.issues.push(
                                        Issue::LinkToNonExistingAnchorInExistingDocument {
                                            location: Location {
                                                file: doc.relative_path.clone(),
                                                line,
                                                start,
                                                end,
                                            },
                                            target_file: target_file.clone(),
                                            anchor: target_anchor,
                                        },
                                    );
                                } else {
                                    result.issues.push(Issue::LinkToNonExistingFile {
                                        location: Location {
                                            file: doc.relative_path.clone(),
                                            line,
                                            start,
                                            end,
                                        },
                                        target,
                                    });
                                    continue;
                                }
                            }
                            result
                                .incoming_doc_links
                                .add(&target_file, doc.relative_path.clone());
                            result
                                .outgoing_doc_links
                                .add(doc.relative_path.clone(), &target_file);
                        }
                        FileType::Resource => {
                            result.outgoing_resource_links.push(target_file);
                        }
                        FileType::Configuration | FileType::Ignored => {}
                    }
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
                                file: doc.relative_path.clone(),
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

/// indicates whether the given Vec<String> contains the given &str
///
// NOTE: cannot use "contains" because https://github.com/rust-lang/rust/issues/42671
fn strings_contain(targets: &[String], target: &str) -> bool {
    targets.iter().any(|t| t == target)
}

#[cfg(test)]
mod tests {

    mod scan {
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
            let want = vec![Issue::LinkToNonExistingFile {
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
        fn link_to_non_existing_anchor_in_existing_file() {
            let dir = test::tmp_dir();
            test::create_file("1.md", "# One\n[non-existing anchor](2.md#zonk)\n", &dir);
            test::create_file("2.md", "# Two\n[One](1.md)", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            let want = vec![Issue::LinkToNonExistingAnchorInExistingDocument {
                location: Location {
                    file: "1.md".into(),
                    line: 1,
                    start: 0,
                    end: 32,
                },
                target_file: "2.md".into(),
                anchor: "zonk".into(),
            }];
            pretty::assert_eq!(have.issues, want);
            assert_eq!(have.incoming_doc_links.data.len(), 2);
            assert_eq!(have.outgoing_doc_links.data.len(), 2);
            assert_eq!(have.outgoing_resource_links.len(), 0);
        }

        #[test]
        fn link_to_non_existing_anchor_in_current_file() {
            let dir = test::tmp_dir();
            test::create_file("1.md", "# One\n[non-existing anchor](#zonk)\n", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            let want = vec![Issue::LinkToNonExistingAnchorInCurrentDocument {
                location: Location {
                    file: "1.md".into(),
                    line: 1,
                    start: 0,
                    end: 28,
                },
                anchor: "zonk".into(),
            }];
            pretty::assert_eq!(have.issues, want);
            assert_eq!(have.incoming_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_resource_links.len(), 0);
        }

        #[test]
        fn link_to_anchor_in_nonexisting_file() {
            let dir = test::tmp_dir();
            test::create_file(
                "1.md",
                "# One\n[anchor in non-existing file](2.md#foo)\n",
                &dir,
            );
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            let want = vec![Issue::LinkToNonExistingFile {
                location: Location {
                    file: "1.md".into(),
                    line: 1,
                    start: 0,
                    end: 39,
                },
                target: "2.md#foo".into(),
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
                working link to [Two](2.md)
                ### section
                working link to [Three](3.md)
                "};
            test::create_file("1.md", content, &dir);
            test::create_file("2.md", "# Two\n[1](1.md)", &dir);
            test::create_file("3.md", "# Three\n[1](1.md)", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            pretty::assert_eq!(have.issues, vec![]);
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
        fn link_without_target() {
            let dir = test::tmp_dir();
            test::create_file("one.md", "# One\n\n[invalid]()\n", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            pretty::assert_eq!(
                have.issues,
                vec![Issue::LinkWithoutTarget {
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
        fn link_to_external_url() {
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
        fn imagelink_to_existing_image() {
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
        fn imagelink_to_non_existing_image() {
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

        #[test]
        fn link_to_existing_resource() {
            let dir = test::tmp_dir();
            test::create_file("1.md", "# One\n\n[docs](docs.pdf)\n", &dir);
            test::create_file("docs.pdf", "PDF content", &dir);
            let base = Tikibase::load(dir, &Config::default()).unwrap();
            let have = scan(&base);
            pretty::assert_eq!(have.issues, vec![]);
            assert_eq!(have.outgoing_resource_links.len(), 1);
            assert_eq!(have.outgoing_resource_links[0], "docs.pdf");
            assert_eq!(have.incoming_doc_links.data.len(), 0);
            assert_eq!(have.outgoing_doc_links.data.len(), 0);
        }
    }

    #[test]
    fn strings_contain() {
        let strings = vec!["1".to_string(), "2".to_string()];
        assert!(super::strings_contain(&strings, "1"));
        assert!(!super::strings_contain(&strings, "3"));
    }
}
