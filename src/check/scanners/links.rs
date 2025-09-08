use cucumber::WriterExt;

use crate::check::{Issue, Location};
use crate::database::{Directory, Document, EntryType, paths};

/// populates the given issues list with all link issues in this document
pub fn scan(
  doc: &Document,
  dir: &Directory,
  issues: &mut Vec<Issue>,
  linked_resources: &mut Vec<String>,
  root: &Directory,
) {
  if dir.config.check_standalone_docs() && doc.links.is_empty() && doc.images.is_empty() {
    issues.push(Issue::DocumentWithoutLinks {
      location: Location {
        file: doc.relative_path.clone(),
        line: 0,
        start: 0,
        end: 0,
      },
    });
  }
  for link in &doc.links {
    if link.target.is_empty() {
      issues.push(Issue::LinkWithoutTarget {
        location: Location {
          file: doc.relative_path.clone(),
          line: link.line.to_owned(),
          start: link.start.to_owned(),
          end: link.end.to_owned(),
        },
      });
      continue;
    }
    if link.target.starts_with("http") {
      // ignore external links
      continue;
    }
    let (target_file, target_anchor) = match link.target.split_once('#') {
      Some((base, anchor)) => (base.to_owned(), format!("#{anchor}")),
      None => (link.target.clone(), String::new()),
    };
    let target_relative_path = paths::join(&dir.relative_path, &target_file);
    let Ok(target_relative_path) = paths::normalize(&target_relative_path) else {
      issues.push(Issue::PathEscapesRoot {
        path: target_relative_path,
        location: Location {
          file: doc.relative_path.clone(),
          line: link.line.to_owned(),
          start: link.start.to_owned(),
          end: link.end.to_owned(),
        },
      });
      continue;
    };
    if target_relative_path == doc.relative_path {
      issues.push(Issue::LinkToSameDocument {
        location: Location {
          file: doc.relative_path.clone(),
          line: link.line.to_owned(),
          start: link.start.to_owned(),
          end: link.end.to_owned(),
        },
      });
      continue;
    }
    if link.target.starts_with('#') {
      if !doc.has_anchor(&link.target) {
        issues.push(Issue::LinkToNonExistingAnchorInCurrentDocument {
          location: Location {
            file: doc.relative_path.clone(),
            line: link.line.to_owned(),
            start: link.start.to_owned(),
            end: link.end.to_owned(),
          },
          anchor: link.target.clone(),
        });
      }
      continue;
    }
    match EntryType::from_str(&target_relative_path) {
      EntryType::Document => {
        if let Some(other_doc) = root.get_doc(&target_relative_path) {
          if !target_anchor.is_empty() && !other_doc.has_anchor(&target_anchor) {
            issues.push(Issue::LinkToNonExistingAnchorInExistingDocument {
              location: Location {
                file: doc.relative_path.clone(),
                line: link.line.to_owned(),
                start: link.start.to_owned(),
                end: link.end.to_owned(),
              },
              target_file: target_relative_path.clone(),
              anchor: target_anchor,
            });
          }
          // check for backlink from doc to us
          if dir.config.bidi_links == Some(true) {
            let link_from_other_to_doc =
              paths::relative(&other_doc.relative_path, &doc.relative_path);
            if !other_doc.contains_reference_to(&link_from_other_to_doc) {
              issues.push(Issue::MissingLink {
                location: Location {
                  file: target_relative_path,
                  line: other_doc.lines_count(),
                  start: 0,
                  end: 0,
                },
                path: link_from_other_to_doc,
                title: doc.human_title().into(),
              });
            }
          }
        } else {
          issues.push(Issue::LinkToNonExistingFile {
            location: Location {
              file: doc.relative_path.clone(),
              line: link.line.to_owned(),
              start: link.start.to_owned(),
              end: link.end.to_owned(),
            },
            target: target_relative_path,
          });
        };
      }
      EntryType::Resource => {
        if root.has_resource(&target_relative_path) {
          linked_resources.push(target_relative_path);
        } else {
          issues.push(Issue::LinkToNonExistingFile {
            location: Location {
              file: doc.relative_path.clone(),
              line: link.line.to_owned(),
              start: link.start.to_owned(),
              end: link.end.to_owned(),
            },
            target: target_relative_path,
          });
        }
      }
      EntryType::Configuration | EntryType::Ignored => {}
      EntryType::Directory => {
        let target_dir = &target_relative_path[..target_relative_path.len() - 1];
        if !root.has_dir(target_dir) {
          issues.push(Issue::LinkToNonExistingDir {
            location: Location {
              file: doc.relative_path.clone(),
              line: link.line.to_owned(),
              start: link.start.to_owned(),
              end: link.end.to_owned(),
            },
            target: target_dir.into(),
          });
        }
      }
    }
  }

  for image in &doc.images {
    if image.src.starts_with("http") {
      continue;
    }
    let target_relative_path = dir.relative_path.join(&image.src);
    let k(target_relative_path) = target_relative_path.0.normalized() else {
      issues.push(Issue::PathEscapesRoot {
        path: target_relative_path,
        location: Location {
          file: doc.relative_path.clone(),
          line: image.line.to_owned(),
          start: image.start.to_owned(),
          end: image.end.to_owned(),
        },
      });
      continue;
    };
    if root.has_resource(&target_relative_path) {
      linked_resources.push(target_relative_path);
    } else {
      issues.push(Issue::BrokenImage {
        location: Location {
          file: doc.relative_path.clone(),
          line: image.line.to_owned(),
          start: image.start.to_owned(),
          end: image.end.to_owned(),
        },
        target: image.src.clone(),
      });
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::check::{Issue, Location};
  use crate::domain::PathRelativeToRoot;
  use crate::{Tikibase, test};
  use big_s::S;
  use indoc::indoc;

  #[test]
  fn link_to_non_existing_file() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file(
      "one.md",
      "# One\n\n[invalid](non-existing.md)\n",
      dir.path(),
    );
    let base = Tikibase::load(dir.path()).unwrap();
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
        file: PathRelativeToRoot::from("one.md"),
        line: 2,
        start: 0,
        end: 26,
      },
      target: S("non-existing.md"),
    }];
    pretty::assert_eq!(issues, want);
    assert_eq!(linked_resources, Vec::<String>::new());
  }

  #[test]
  fn link_to_non_existing_anchor_in_existing_file() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file(
      "1.md",
      "# One\n[non-existing anchor](2.md#zonk)\n",
      dir.path(),
    );
    test::create_file("2.md", "# Two\n[One](1.md)", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
        file: PathRelativeToRoot::from("1.md"),
        line: 1,
        start: 0,
        end: 32,
      },
      target_file: S("2.md"),
      anchor: S("#zonk"),
    }];
    pretty::assert_eq!(issues, want);
    assert_eq!(linked_resources, Vec::<String>::new());
  }

  #[test]
  fn link_to_non_existing_anchor_in_current_file() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file("1.md", "# One\n[non-existing anchor](#zonk)\n", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
        file: S("1.md"),
        line: 1,
        start: 0,
        end: 28,
      },
      anchor: S("#zonk"),
    }];
    pretty::assert_eq!(issues, want);
    assert_eq!(linked_resources, Vec::<String>::new());
  }

  #[test]
  fn link_to_existing_anchor_in_current_file() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file(
      "1.md",
      "# One\n[existing anchor](#section)\n### section\ntext",
      dir.path(),
    );
    let base = Tikibase::load(dir.path()).unwrap();
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
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file(
      "1.md",
      "# One\n[anchor in non-existing file](2.md#foo)\n",
      dir.path(),
    );
    let base = Tikibase::load(dir.path()).unwrap();
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
        file: PathRelativeToRoot::from("1.md"),
        line: 1,
        start: 0,
        end: 39,
      },
      target: S("2.md"),
    }];
    pretty::assert_eq!(issues, want);
    assert_eq!(linked_resources, Vec::<String>::new());
  }

  #[test]
  fn link_to_existing_file_bidi() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file("tikibase.json", "{ \"bidiLinks\": true }", dir.path());
    let content = indoc! {"
                # One
                working link to [Two](two/2.md)
                ### section
                working link to [Three](three/3.md)
                "};
    test::create_file("1.md", content, dir.path());
    test::create_file("two/2.md", "# Two\n[One](../1.md)", dir.path());
    test::create_file("three/3.md", "# Three\n[One](../1.md)", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
    let dir = camino_tempfile::tempdir().unwrap();
    let content = indoc! {"
                # One
                working link to [Two](two/2.md)
                "};
    test::create_file("1.md", content, dir.path());
    test::create_file("two/2.md", "# Two\n[One](../1.md)", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
  fn missing_backlink() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file("tikibase.json", "{ \"bidiLinks\": true }", dir.path());
    let content = indoc! {"
                # One
                working link to [Two](two/2.md)
                ### section
                working link to [Three](three/3.md)
                "};
    test::create_file("1.md", content, dir.path());
    test::create_file("two/2.md", "# Two\n[One](../1.md)", dir.path());
    test::create_file("three/3.md", "# Three", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
      vec![Issue::MissingLink {
        location: Location {
          file: S("three/3.md"),
          line: 0,
          start: 0,
          end: 0,
        },
        path: S("../1.md"),
        title: S("One"),
      }]
    );
    assert_eq!(linked_resources, Vec::<String>::new());
  }

  #[test]
  fn link_within_subdir() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file("sub/1.md", "# One\n[two](2.md)", dir.path());
    test::create_file("sub/2.md", "# Two\n[one](1.md)", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
    let doc = base.get_doc("sub/1.md").unwrap();
    let mut issues = vec![];
    let mut linked_resources = vec![];
    super::scan(
      doc,
      base.dir.dirs.get("sub").unwrap(),
      &mut issues,
      &mut linked_resources,
      &base.dir,
    );
    pretty::assert_eq!(issues, vec![]);
    assert_eq!(linked_resources, Vec::<String>::new());
  }

  #[test]
  fn link_to_doc_in_parent_dir() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file("sub/1.md", "# One\n[two](../zonk.md)", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
    let doc = base.get_doc("sub/1.md").unwrap();
    let mut issues = vec![];
    let mut linked_resources = vec![];
    super::scan(
      doc,
      base.dir.dirs.get("sub").unwrap(),
      &mut issues,
      &mut linked_resources,
      &base.dir,
    );
    pretty::assert_eq!(
      issues,
      vec![Issue::LinkToNonExistingFile {
        location: Location {
          file: S("sub/1.md"),
          line: 1,
          start: 0,
          end: 17,
        },
        target: S("zonk.md")
      }]
    );
    assert_eq!(linked_resources, Vec::<String>::new());
  }

  #[test]
  fn link_to_existing_dir() {
    let dir = camino_tempfile::tempdir().unwrap();
    let content = indoc! {"
                # One
                working link to [dir](dir/)
                "};
    test::create_file("1.md", content, dir.path());
    test::create_file("dir/2.md", "# Two", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
    let dir = camino_tempfile::tempdir().unwrap();
    let content = indoc! {"
                # One
                link to non-existing dir [zonk](zonk/)
                "};
    test::create_file("1.md", content, dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
          file: S("1.md"),
          line: 1,
          start: 25,
          end: 38,
        },
        target: S("zonk"),
      }]
    );
    assert_eq!(linked_resources, Vec::<String>::new());
  }

  #[test]
  fn link_without_target() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file("one.md", "# One\n\n[invalid]()\n", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
          file: S("one.md"),
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
    let dir = camino_tempfile::tempdir().unwrap();
    let content = indoc! {"
                # One

                [external site](https://google.com)
                ![external image](https://google.com/foo.png)
                "};
    test::create_file("one.md", content, dir.path());
    test::create_file("two.md", "# Two\n[one](one.md)", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
  fn imagelink_to_existing_image_in_subdir() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file("one/two/1.md", "# One\n\n![image](foo.png)\n", dir.path());
    test::create_file("one/two/foo.png", "image content", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
    let doc = base.get_doc("one/two/1.md").unwrap();
    let dir = base.get_dir("one/two").unwrap();
    let mut issues = vec![];
    let mut linked_resources = vec![];
    super::scan(doc, dir, &mut issues, &mut linked_resources, &base.dir);
    assert!(issues.is_empty());
    assert_eq!(linked_resources, vec![S("one/two/foo.png")]);
  }

  #[test]
  fn imagelink_to_existing_image() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file("1.md", "# One\n\n![image](foo.png)\n", dir.path());
    test::create_file("foo.png", "image content", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
    assert_eq!(linked_resources, vec![S("foo.png")]);
  }

  #[test]
  fn imagelink_to_non_existing_image() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file("1.md", "# One\n\n![image](zonk.png)\n", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
        file: S("1.md"),
        line: 2,
        start: 0,
        end: 18,
      },
      target: S("zonk.png"),
    }];
    pretty::assert_eq!(issues, want);
    assert_eq!(linked_resources, Vec::<String>::new());
  }

  #[test]
  fn link_to_existing_resource() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file("1.md", "# One\n\n[doc](doc.pdf)\n", dir.path());
    test::create_file("doc.pdf", "PDF content", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
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
    assert_eq!(linked_resources, vec!["doc.pdf"]);
  }

  #[test]
  fn link_to_existing_resource_in_subfolder() {
    let dir = camino_tempfile::tempdir().unwrap();
    test::create_file("sub/1.md", "# One\n\n[doc](doc.pdf)\n", dir.path());
    test::create_file("sub/doc.pdf", "PDF content", dir.path());
    let base = Tikibase::load(dir.path()).unwrap();
    let doc = base.get_doc("sub/1.md").unwrap();
    let mut issues = vec![];
    let mut linked_resources = vec![];
    let subdir = base.get_dir("sub").unwrap();
    super::scan(doc, subdir, &mut issues, &mut linked_resources, &base.dir);
    pretty::assert_eq!(issues, vec![]);
    assert_eq!(linked_resources, vec!["sub/doc.pdf"]);
  }
}
