use crate::{Document, Issue, Location};

/// populates the given issues list with all sections in this document that don't match the configured sections
pub fn scan(doc: &Document, issues: &mut Vec<Issue>) {
    let footnotes = match doc.footnotes() {
        Ok(footnotes) => footnotes,
        Err(issue) => {
            issues.push(issue);
            return;
        }
    };
    for missing_reference in footnotes.missing_references() {
        issues.push(Issue::MissingFootnote {
            location: Location {
                file: doc.relative_path.clone(),
                line: missing_reference.line,
                start: missing_reference.start,
                end: missing_reference.end,
            },
            identifier: missing_reference.identifier.clone(),
        });
    }
    for unused_definition in footnotes.unused_definitions() {
        issues.push(Issue::UnusedFootnote {
            location: Location {
                file: doc.relative_path.clone(),
                line: unused_definition.line,
                start: unused_definition.start,
                end: unused_definition.end,
            },
            identifier: unused_definition.identifier.clone(),
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{Document, Issue, Location};
    use indoc::indoc;
    use std::path::PathBuf;

    #[test]
    fn missing_footnote_definition() {
        let content = indoc! {"
                # Title
                existing footnote[^existing]
                other footnote[^other]

                ```go
                result := map[^0]
                ```

                Another snippet of code that should be ignored: `map[^0]`.

                ### links

                [^existing]: existing footnote
                "};
        let doc = Document::from_str("test.md", content).unwrap();
        let mut have = vec![];
        super::scan(&doc, &mut have);
        let want = vec![Issue::MissingFootnote {
            location: Location {
                file: PathBuf::from("test.md"),
                line: 2,
                start: 14,
                end: 22,
            },
            identifier: "other".into(),
        }];
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn unused_footnote_definition() {
        let content = indoc! {"
                # Title
                existing footnote[^existing]

                ```go
                result := map[^0]
                ```

                Another snippet of code that should be ignored: `map[^0]`.

                ### links

                [^existing]: existing footnote
                [^unused]: unused footnote
                "};
        let doc = Document::from_str("test.md", content).unwrap();
        let mut have = vec![];
        super::scan(&doc, &mut have);
        let want = vec![Issue::UnusedFootnote {
            location: Location {
                file: PathBuf::from("test.md"),
                line: 12,
                start: 0,
                end: 10,
            },
            identifier: "unused".into(),
        }];
        pretty::assert_eq!(have, want);
    }
}
