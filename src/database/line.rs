use crate::database::{Footnote, Footnotes, Reference};
use crate::{Issue, Location};
use once_cell::sync::Lazy;
use regex::Regex;
use std::path::Path;

#[derive(Debug, Default, PartialEq)]
pub struct Line(String);

static MD_RE: Lazy<Regex> = Lazy::new(|| Regex::new("(!?)\\[[^\\]]*\\]\\(([^)]*)\\)").unwrap());
static A_HTML_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<a href="(.*)">(.*)</a>"#).unwrap());
static IMG_HTML_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<img src="([^"]*)"[^>]*>"#).unwrap());
static FOOTNOTE_RE: Lazy<Regex> = Lazy::new(|| Regex::new("\\[\\^(\\w+)\\](:?)").unwrap());

impl Line {
    /// appends all footnote definitions and references to the given result structure
    ///
    /// This is implemented using a mutable accumulator parameter to minimize memory allocations
    /// since this code is running for every line in a Tikibase, i.e. potentially hundreds of thousands of times.
    pub fn add_footnotes_to(
        &self,
        result: &mut Footnotes,
        file: &Path,
        line: u32,
    ) -> Result<(), Issue> {
        let sanitized = sanitize_code_segments(&self.0, file, line)?;
        for captures in FOOTNOTE_RE.captures_iter(&sanitized) {
            let total_match = captures.get(0).unwrap();
            let footnote = Footnote {
                identifier: captures.get(1).unwrap().as_str().to_string(),
                line,
                start: total_match.start() as u32,
                end: total_match.end() as u32,
            };
            match captures[2].is_empty() {
                false => result.definitions.push(footnote),
                true => result.references.push(footnote),
            };
        }
        Ok(())
    }

    pub fn from<S: Into<String>>(text: S) -> Line {
        Line(text.into())
    }

    /// indicates whether this line is the beginning or end of a code block
    pub fn is_code_block_boundary(&self) -> bool {
        self.text().starts_with("```")
    }

    /// provides all links and images in this line
    // TODO: reuse shared global Vec here
    pub fn references(&self) -> Vec<Reference> {
        let mut result = Vec::new();
        for cap in MD_RE.captures_iter(&self.0) {
            let full_match = cap.get(0).unwrap();
            match &cap[1] {
                "!" => result.push(Reference::Image {
                    src: cap[2].to_string(),
                    start: full_match.start() as u32,
                    end: full_match.end() as u32,
                }),
                "" => {
                    let mut destination = cap[2].to_string();
                    if let Some(idx) = destination.find('#') {
                        destination.truncate(idx);
                    }
                    result.push(Reference::Link {
                        destination,
                        start: full_match.start() as u32,
                        end: full_match.end() as u32,
                    });
                }
                _ => panic!("unexpected capture: '{}'", &cap[1]),
            }
        }
        for cap in A_HTML_RE.captures_iter(&self.0) {
            let full_match = cap.get(0).unwrap();
            result.push(Reference::Link {
                destination: cap[1].to_string(),
                start: full_match.start() as u32,
                end: full_match.end() as u32,
            });
        }
        for cap in IMG_HTML_RE.captures_iter(&self.0) {
            let full_match = cap.get(0).unwrap();
            result.push(Reference::Image {
                src: cap[1].to_string(),
                start: full_match.start() as u32,
                end: full_match.end() as u32,
            });
        }
        result
    }

    /// provides the text of this line
    pub fn text(&self) -> &str {
        &self.0
    }
}

/// non-destructively overwrites areas inside backticks in the given string with spaces
fn sanitize_code_segments(text: &str, file: &Path, line: u32) -> Result<String, Issue> {
    let mut result = String::with_capacity(text.len());
    let mut code_block_start: Option<u32> = None;
    for (i, c) in text.char_indices() {
        if c == '`' {
            code_block_start = match code_block_start {
                Some(_) => None,
                None => Some(i as u32),
            };
            result.push(c);
            continue;
        }
        result.push(match code_block_start {
            Some(_) => ' ',
            None => c,
        });
    }
    if let Some(code_block_start) = code_block_start {
        return Err(Issue::UnclosedBacktick {
            location: Location {
                file: file.into(),
                line,
                start: code_block_start as u32,
                end: text.len() as u32,
            },
        });
    }
    Ok(result)
}

#[cfg(test)]
mod tests {

    mod add_footnotes_to {
        use crate::database::{Footnote, Footnotes, Line};
        use std::path::Path;

        #[test]
        fn none() {
            let line = Line::from("text");
            let mut have = Footnotes::default();
            line.add_footnotes_to(&mut have, Path::new(""), 0).unwrap();
            let want = Footnotes::default();
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn references() {
            let line = Line::from("- text [^1] [^2]");
            let mut have = Footnotes::default();
            line.add_footnotes_to(&mut have, Path::new(""), 0).unwrap();
            let want = Footnotes {
                definitions: vec![],
                references: vec![
                    Footnote {
                        line: 0,
                        identifier: "1".into(),
                        start: 7,
                        end: 11,
                    },
                    Footnote {
                        line: 0,
                        identifier: "2".into(),
                        start: 12,
                        end: 16,
                    },
                ],
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn definitions() {
            let line = Line::from("[^1]: the one\nother");
            let mut have = Footnotes::default();
            line.add_footnotes_to(&mut have, Path::new(""), 0).unwrap();
            let want = Footnotes {
                definitions: vec![Footnote {
                    identifier: "1".into(),
                    line: 0,
                    start: 0,
                    end: 5,
                }],
                references: vec![],
            };
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn ignore_code_looking_like_footnotes() {
            let line = Line::from("the code `map[^0]`");
            let mut have = Footnotes::default();
            line.add_footnotes_to(&mut have, Path::new(""), 0).unwrap();
            let want = Footnotes::default();
            pretty::assert_eq!(have, want);
        }
    }

    mod is_code_block_boundary {
        use crate::database::Line;

        #[test]
        fn no_boundary() {
            let line = Line::from("foo");
            let have = line.is_code_block_boundary();
            assert!(!have)
        }

        #[test]
        fn plain_boundary() {
            let line = Line::from("```");
            let have = line.is_code_block_boundary();
            assert!(have)
        }

        #[test]
        fn boundary_with_language() {
            let line = Line::from("```rs");
            let have = line.is_code_block_boundary();
            assert!(have)
        }
    }

    mod references {
        use super::super::Reference;
        use crate::database::Line;

        #[test]
        fn link_md() {
            let line = Line::from(
                r#"an MD link: [one](one.md) and one to a section: [two pieces](two.md#pieces)!"#,
            );
            let have = line.references();
            let want = vec![
                Reference::Link {
                    destination: "one.md".into(),
                    start: 12,
                    end: 25,
                },
                Reference::Link {
                    destination: "two.md".into(),
                    start: 48,
                    end: 75,
                },
            ];
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn link_html() {
            let line = Line::from(r#"an HTML link: <a href="two.md">two</a>"#);
            let have = line.references();
            let want = vec![Reference::Link {
                destination: "two.md".into(),
                start: 14,
                end: 38,
            }];
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn img_md() {
            let line = Line::from(r#"an MD image: ![zonk](zonk.md)"#);
            let have = line.references();
            let want = vec![Reference::Image {
                src: "zonk.md".into(),
                start: 13,
                end: 29,
            }];
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn img_html() {
            let line = Line::from(r#"<img src="zonk.md">"#);
            let have = line.references();
            let want = vec![Reference::Image {
                src: "zonk.md".into(),
                start: 0,
                end: 19,
            }];
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn img_html_extra_attributes() {
            let line = Line::from(r#"<img src="zonk.md" width="10" height="10">"#);
            let have = line.references();
            let want = vec![Reference::Image {
                src: "zonk.md".into(),
                start: 0,
                end: 42,
            }];
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn img_xml_nospace() {
            let line = Line::from(r#"<img src="zonk.md"/>"#);
            let have = line.references();
            let want = vec![Reference::Image {
                src: "zonk.md".into(),
                start: 0,
                end: 20,
            }];
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn img_xml_space() {
            let line = Line::from(r#"<img src="zonk.md" />"#);
            let have = line.references();
            let want = vec![Reference::Image {
                src: "zonk.md".into(),
                start: 0,
                end: 21,
            }];
            pretty::assert_eq!(have, want)
        }
    }

    mod sanitize_code_segments {
        use super::super::sanitize_code_segments;
        use crate::{Issue, Location};
        use std::path::{Path, PathBuf};

        #[test]
        fn with_code_blocks() {
            let give = "one `map[0]` two `more code` three";
            let want = "one `      ` two `         ` three".to_string();
            assert_eq!(sanitize_code_segments(give, Path::new(""), 0), Ok(want))
        }

        #[test]
        fn empty_string() {
            let give = "";
            let want = "".to_string();
            assert_eq!(sanitize_code_segments(give, Path::new(""), 0), Ok(want))
        }

        #[test]
        fn unclosed_backtick() {
            let give = "one `unclosed";
            let want = Err(Issue::UnclosedBacktick {
                location: Location {
                    file: PathBuf::from(""),
                    line: 12,
                    start: 4,
                    end: 13,
                },
            });
            let have = sanitize_code_segments(give, Path::new(""), 12);
            assert_eq!(have, want)
        }
    }
}
