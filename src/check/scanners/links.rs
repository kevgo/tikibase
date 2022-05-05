use crate::check::{Issue, Location};
use crate::database::{paths, Directory, Document, EntryType, Reference};

/// populates the given issues list with all link issues in this document
pub fn scan(
    doc: &Document,
    dir: &Directory,
    issues: &mut Vec<Issue>,
    linked_resources: &mut Vec<String>,
    root: &Directory,
) {
    if doc.references.is_empty() {
        issues.push(Issue::DocumentWithoutLinks {
            location: Location {
                file: doc.relative_path.clone(),
                line: 0,
                start: 0,
                end: 0,
            },
        });
    }
    for reference in &doc.references {
        match reference {
            Reference::Link {
                target,
                line,
                start,
                end,
            } => {
                if target.is_empty() {
                    issues.push(Issue::LinkWithoutTarget {
                        location: Location {
                            file: doc.relative_path.clone(),
                            line: line.to_owned(),
                            start: start.to_owned(),
                            end: end.to_owned(),
                        },
                    });
                    continue;
                }
                if target.starts_with("http") {
                    // ignore external links
                    continue;
                }
                let (target_file, target_anchor) = match target.split_once('#') {
                    Some((base, anchor)) => (base.to_string(), format!("#{}", anchor)),
                    None => (target.clone(), "".to_string()),
                };
                let target_file = if let Ok(target) =
                    paths::normalize(&paths::join(&dir.relative_path, &target_file))
                {
                    target
                } else {
                    issues.push(Issue::PathEscapesRoot {
                        path: paths::join(&dir.relative_path, &target_file),
                        location: Location {
                            file: doc.relative_path.clone(),
                            line: line.to_owned(),
                            start: start.to_owned(),
                            end: end.to_owned(),
                        },
                    });
                    continue;
                };
                if target_file == doc.relative_path {
                    issues.push(Issue::LinkToSameDocument {
                        location: Location {
                            file: doc.relative_path.clone(),
                            line: line.to_owned(),
                            start: start.to_owned(),
                            end: end.to_owned(),
                        },
                    });
                    continue;
                }
                if target.starts_with('#') {
                    if !doc.has_anchor(target) {
                        issues.push(Issue::LinkToNonExistingAnchorInCurrentDocument {
                            location: Location {
                                file: doc.relative_path.clone(),
                                line: line.to_owned(),
                                start: start.to_owned(),
                                end: end.to_owned(),
                            },
                            anchor: target.clone(),
                        });
                    }
                    continue;
                }
                match EntryType::from_str(&target_file) {
                    EntryType::Document => {
                        if let Some(other_doc) = root.get_doc(&target_file) {
                            if !target_anchor.is_empty() && !other_doc.has_anchor(&target_anchor) {
                                issues.push(Issue::LinkToNonExistingAnchorInExistingDocument {
                                    location: Location {
                                        file: doc.relative_path.clone(),
                                        line: line.to_owned(),
                                        start: start.to_owned(),
                                        end: end.to_owned(),
                                    },
                                    target_file: target_file.clone(),
                                    anchor: target_anchor,
                                });
                            }
                            // check for backlink from doc to us
                            if let Some(bidi_links) = dir.config.bidi_links {
                                if bidi_links {
                                    let link_from_other_to_doc = paths::relative(
                                        &other_doc.relative_path,
                                        &doc.relative_path,
                                    );
                                    if !other_doc.contains_reference_to(&link_from_other_to_doc) {
                                        issues.push(Issue::MissingLink {
                                            location: Location {
                                                file: target_file,
                                                line: other_doc.lines_count(),
                                                start: 0,
                                                end: 0,
                                            },
                                            path: doc.relative_path.clone(),
                                            title: doc.human_title().into(),
                                        });
                                    }
                                }
                            }
                        } else {
                            issues.push(Issue::LinkToNonExistingFile {
                                location: Location {
                                    file: doc.relative_path.clone(),
                                    line: line.to_owned(),
                                    start: start.to_owned(),
                                    end: end.to_owned(),
                                },
                                target: target.into(),
                            });
                        };
                    }
                    EntryType::Resource => {
                        if root.has_resource(&target_file) {
                            linked_resources.push(paths::join(&dir.relative_path, &target_file));
                        } else {
                            issues.push(Issue::LinkToNonExistingFile {
                                location: Location {
                                    file: doc.relative_path.clone(),
                                    line: line.to_owned(),
                                    start: start.to_owned(),
                                    end: end.to_owned(),
                                },
                                target: target.into(),
                            });
                        }
                    }
                    EntryType::Configuration | EntryType::Ignored => {}
                    EntryType::Directory => {
                        let target_dir = &target_file[..target_file.len() - 1];
                        if !root.has_dir(target_dir) {
                            issues.push(Issue::LinkToNonExistingDir {
                                location: Location {
                                    file: doc.relative_path.clone(),
                                    line: line.to_owned(),
                                    start: start.to_owned(),
                                    end: end.to_owned(),
                                },
                                target: target_dir.into(),
                            });
                        }
                    }
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
                if root.has_resource(&src) {
                    linked_resources.push(paths::join(&dir.relative_path, src));
                } else {
                    issues.push(Issue::BrokenImage {
                        location: Location {
                            file: doc.relative_path.clone(),
                            line: line.to_owned(),
                            start: start.to_owned(),
                            end: end.to_owned(),
                        },
                        target: src.clone(),
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::check::{Issue, Location};
    use crate::{test, Tikibase};
    use indoc::indoc;

    #[test]
    fn link_to_non_existing_file() {
        let dir = test::tmp_dir();
        test::create_file("one.md", "# One\n\n[invalid](non-existing.md)\n", &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("one.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        let want = vec![Issue::LinkToNonExistingFile {
            location: Location {
                file: "one.md".into(),
                line: 2,
                start: 0,
                end: 26,
            },
            target: "non-existing.md".into(),
        }];
        pretty::assert_eq!(issues, want);
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn link_to_non_existing_anchor_in_existing_file() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n[non-existing anchor](2.md#zonk)\n", &dir);
        test::create_file("2.md", "# Two\n[One](1.md)", &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        let want = vec![Issue::LinkToNonExistingAnchorInExistingDocument {
            location: Location {
                file: "1.md".into(),
                line: 1,
                start: 0,
                end: 32,
            },
            target_file: "2.md".into(),
            anchor: "#zonk".into(),
        }];
        pretty::assert_eq!(issues, want);
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn link_to_non_existing_anchor_in_current_file() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n[non-existing anchor](#zonk)\n", &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        let want = vec![Issue::LinkToNonExistingAnchorInCurrentDocument {
            location: Location {
                file: "1.md".into(),
                line: 1,
                start: 0,
                end: 28,
            },
            anchor: "#zonk".into(),
        }];
        pretty::assert_eq!(issues, want);
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn link_to_existing_anchor_in_current_file() {
        let dir = test::tmp_dir();
        test::create_file(
            "1.md",
            "# One\n[existing anchor](#section)\n### section\ntext",
            &dir,
        );
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        let want = vec![];
        pretty::assert_eq!(issues, want);
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn link_to_anchor_in_nonexisting_file() {
        let dir = test::tmp_dir();
        test::create_file(
            "1.md",
            "# One\n[anchor in non-existing file](2.md#foo)\n",
            &dir,
        );
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        let want = vec![Issue::LinkToNonExistingFile {
            location: Location {
                file: "1.md".into(),
                line: 1,
                start: 0,
                end: 39,
            },
            target: "2.md#foo".into(),
        }];
        pretty::assert_eq!(issues, want);
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn link_to_existing_file_bidi() {
        let dir = test::tmp_dir();
        test::create_file("tikibase.json", "{ \"bidiLinks\": true }", &dir);
        let content = indoc! {"
                # One
                working link to [Two](two/2.md)
                ### section
                working link to [Three](three/3.md)
                "};
        test::create_file("1.md", content, &dir);
        test::create_file("two/2.md", "# Two\n[One](../1.md)", &dir);
        test::create_file("three/3.md", "# Three\n[One](../2.md)", &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        pretty::assert_eq!(issues, vec![]);
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn link_to_existing_file_no_bidi() {
        let dir = test::tmp_dir();
        let content = indoc! {"
                # One
                working link to [Two](two/2.md)
                ### section
                working link to [Three](three/3.md)
                "};
        test::create_file("1.md", content, &dir);
        test::create_file("two/2.md", "# Two\n[One](../1.md)", &dir);
        test::create_file("three/3.md", "# Three\n[One](../1.md)", &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        pretty::assert_eq!(issues, vec![]);
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn link_within_subdir() {
        let dir = test::tmp_dir();
        test::create_file("sub/1.md", "# One\n[two](2.md)", &dir);
        test::create_file("sub/2.md", "# Two\n[one](1.md)", &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("sub/1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir.dirs.get("sub").unwrap(),
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        pretty::assert_eq!(issues, vec![]);
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn link_to_existing_dir() {
        let dir = test::tmp_dir();
        let content = indoc! {"
                # One
                working link to [dir](dir/)
                "};
        test::create_file("1.md", content, &dir);
        test::create_file("dir/2.md", "# Two", &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        pretty::assert_eq!(issues, vec![]);
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn link_to_non_existing_dir() {
        let dir = test::tmp_dir();
        let content = indoc! {"
                # One
                link to non-existing dir [zonk](zonk/)
                "};
        test::create_file("1.md", content, &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        pretty::assert_eq!(
            issues,
            vec![Issue::LinkToNonExistingDir {
                location: Location {
                    file: "1.md".into(),
                    line: 1,
                    start: 25,
                    end: 38,
                },
                target: "zonk".into(),
            }]
        );
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn link_without_target() {
        let dir = test::tmp_dir();
        test::create_file("one.md", "# One\n\n[invalid]()\n", &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("one.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        pretty::assert_eq!(
            issues,
            vec![Issue::LinkWithoutTarget {
                location: Location {
                    file: "one.md".into(),
                    line: 2,
                    start: 0,
                    end: 11,
                }
            }]
        );
        assert_eq!(linked_resources, Vec::<String>::new());
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
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("one.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        assert!(issues.is_empty());
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn imagelink_to_existing_image() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n\n![image](foo.png)\n", &dir);
        test::create_file("foo.png", "image content", &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        assert!(issues.is_empty());
        assert_eq!(linked_resources, vec!["foo.png".to_string()]);
    }

    #[test]
    fn imagelink_to_non_existing_image() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n\n![image](zonk.png)\n", &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        let want = vec![Issue::BrokenImage {
            location: Location {
                file: "1.md".into(),
                line: 2,
                start: 0,
                end: 18,
            },
            target: "zonk.png".into(),
        }];
        pretty::assert_eq!(issues, want);
        assert_eq!(linked_resources, Vec::<String>::new());
    }

    #[test]
    fn link_to_existing_resource() {
        let dir = test::tmp_dir();
        test::create_file("1.md", "# One\n\n[docs](docs.pdf)\n", &dir);
        test::create_file("docs.pdf", "PDF content", &dir);
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &base.dir,
            &mut issues,
            &mut linked_resources,
            &base.dir,
        );
        pretty::assert_eq!(issues, vec![]);
        assert_eq!(linked_resources, vec!["docs.pdf"]);
    }
}
