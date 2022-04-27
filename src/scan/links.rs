use crate::database::{Directory, EntryType, Reference};
use crate::{Config, Document, Issue, Location};
use std::path::{Path, PathBuf};

/// populates the given issues list with all link issues in this document
pub fn scan(
    doc: &Document,
    path: &Path,
    dir: &Path,
    issues: &mut Vec<Issue>,
    linked_resources: &mut Vec<PathBuf>,
    root: &Directory,
    config: &Config,
) {
    if doc.references.is_empty() {
        issues.push(Issue::DocumentWithoutLinks {
            location: Location {
                file: path.into(),
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
                            file: path.into(),
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
                let path_str = path.to_string_lossy();
                if target_file == path_str {
                    issues.push(Issue::LinkToSameDocument {
                        location: Location {
                            file: path.into(),
                            line: line.to_owned(),
                            start: start.to_owned(),
                            end: end.to_owned(),
                        },
                    });
                    continue;
                }
                if target.starts_with('#')
                    && !doc
                        .content_sections
                        .iter()
                        .any(|section| &section.anchor() == target)
                {
                    issues.push(Issue::LinkToNonExistingAnchorInCurrentDocument {
                        location: Location {
                            file: path.into(),
                            line: line.to_owned(),
                            start: start.to_owned(),
                            end: end.to_owned(),
                        },
                        anchor: target.clone(),
                    });
                    continue;
                }
                match EntryType::from_str(&target_file) {
                    EntryType::Document => {
                        if let Some(other_doc) = root.get_doc(&target_file) {
                            if !target_anchor.is_empty() && !other_doc.has_anchor(&target_anchor) {
                                issues.push(Issue::LinkToNonExistingAnchorInExistingDocument {
                                    location: Location {
                                        file: path.into(),
                                        line: line.to_owned(),
                                        start: start.to_owned(),
                                        end: end.to_owned(),
                                    },
                                    target_file: target_file.clone(),
                                    anchor: target_anchor,
                                });
                                // continue;
                            }
                            // check for backlink from doc to us
                            if let Some(bidi_links) = config.bidi_links {
                                if bidi_links && !other_doc.contains_reference_to(path) {
                                    issues.push(Issue::MissingLink {
                                        location: Location {
                                            file: PathBuf::from(target_file),
                                            line: other_doc.lines_count(),
                                            start: 0,
                                            end: 0,
                                        },
                                        path: path.into(),
                                        title: doc.human_title().into(),
                                    });
                                }
                            }
                        } else {
                            issues.push(Issue::LinkToNonExistingFile {
                                location: Location {
                                    file: path.into(),
                                    line: line.to_owned(),
                                    start: start.to_owned(),
                                    end: end.to_owned(),
                                },
                                target: target.into(),
                            });
                            continue;
                        };
                    }
                    EntryType::Resource => {
                        if !root.has_resource(&target_file) {
                            issues.push(Issue::LinkToNonExistingFile {
                                location: Location {
                                    file: path.into(),
                                    line: line.to_owned(),
                                    start: start.to_owned(),
                                    end: end.to_owned(),
                                },
                                target: target.into(),
                            });
                            continue;
                        }
                        linked_resources.push(dir.join(&target_file));
                    }
                    EntryType::Configuration | EntryType::Ignored => {}
                    EntryType::Directory => todo!(),
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
                if !root.has_resource(&src) {
                    issues.push(Issue::BrokenImage {
                        location: Location {
                            file: path.into(),
                            line: line.to_owned(),
                            start: start.to_owned(),
                            end: end.to_owned(),
                        },
                        target: src.clone(),
                    });
                    continue;
                }
                linked_resources.push(dir.join(src));
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{test, Config, Issue, Location, Tikibase};
    use indoc::indoc;
    use std::path::PathBuf;

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
            &PathBuf::from("one.md"),
            &PathBuf::from(""),
            &mut issues,
            &mut linked_resources,
            &base.dir,
            &Config::default(),
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
        assert_eq!(linked_resources, Vec::<PathBuf>::new());
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
            &PathBuf::from("1.md"),
            &PathBuf::from(""),
            &mut issues,
            &mut linked_resources,
            &base.dir,
            &Config::default(),
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
        assert_eq!(linked_resources, Vec::<PathBuf>::new());
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
            &PathBuf::from("1.md"),
            &PathBuf::from(""),
            &mut issues,
            &mut linked_resources,
            &base.dir,
            &Config::default(),
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
        assert_eq!(linked_resources, Vec::<PathBuf>::new());
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
            &PathBuf::from("1.md"),
            &PathBuf::from(""),
            &mut issues,
            &mut linked_resources,
            &base.dir,
            &Config::default(),
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
        assert_eq!(linked_resources, Vec::<PathBuf>::new());
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
        let base = Tikibase::load(dir).unwrap();
        let doc = base.get_doc("1.md").unwrap();
        let mut issues = vec![];
        let mut linked_resources = vec![];
        super::scan(
            doc,
            &PathBuf::from("1.md"),
            &PathBuf::from(""),
            &mut issues,
            &mut linked_resources,
            &base.dir,
            &Config::default(),
        );
        pretty::assert_eq!(issues, vec![]);
        assert_eq!(linked_resources, Vec::<PathBuf>::new());
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
            &PathBuf::from("one.md"),
            &PathBuf::from(""),
            &mut issues,
            &mut linked_resources,
            &base.dir,
            &Config::default(),
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
        assert_eq!(linked_resources, Vec::<PathBuf>::new());
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
            &PathBuf::from("one.md"),
            &PathBuf::from(""),
            &mut issues,
            &mut linked_resources,
            &base.dir,
            &Config::default(),
        );
        assert!(issues.is_empty());
        assert_eq!(linked_resources, Vec::<PathBuf>::new());
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
            &PathBuf::from("one.md"),
            &PathBuf::from(""),
            &mut issues,
            &mut linked_resources,
            &base.dir,
            &Config::default(),
        );
        assert!(issues.is_empty());
        assert_eq!(linked_resources, vec![PathBuf::from("foo.png")]);
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
            &PathBuf::from("1.md"),
            &PathBuf::from(""),
            &mut issues,
            &mut linked_resources,
            &base.dir,
            &Config::default(),
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
        assert_eq!(linked_resources, Vec::<PathBuf>::new());
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
            &PathBuf::from("1.md"),
            &PathBuf::from(""),
            &mut issues,
            &mut linked_resources,
            &base.dir,
            &Config::default(),
        );
        pretty::assert_eq!(issues, vec![]);
        assert_eq!(linked_resources, vec![PathBuf::from("docs.pdf")]);
    }
}
