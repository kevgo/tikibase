use crate::{Issue, Location, Tikibase};

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    for doc in &base.docs {
        let footnotes = match doc.footnotes() {
            Ok(footnotes) => footnotes,
            Err(issue) => return vec![issue],
        };
        for missing_reference in footnotes.missing_references() {
            issues.push(Issue::MissingFootnote {
                location: Location {
                    file: doc.path.clone(),
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
                    file: doc.path.clone(),
                    line: unused_definition.line,
                    start: unused_definition.start,
                    end: unused_definition.end,
                },
                identifier: unused_definition.identifier.clone(),
            })
        }
    }
    issues
}

#[cfg(test)]
mod tests {
    use crate::{test, Config, Issue, Location, Tikibase};
    use indoc::indoc;
    use std::path::PathBuf;

    #[test]
    fn unused_footnote() {
        let dir = test::tmp_dir();
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
        test::create_file("1.md", content, &dir);
        let base = Tikibase::load(dir, &Config::default()).unwrap();
        let have = super::scan(&base);
        let want = vec![Issue::UnusedFootnote {
            location: Location {
                file: PathBuf::from("1.md"),
                line: 12,
                start: 0,
                end: 10,
            },
            identifier: "unused".into(),
        }];
        pretty::assert_eq!(have, want)
    }
}
