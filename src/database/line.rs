use super::{Reference, SourceReference};
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Default, PartialEq)]
pub struct Line(String);

static MD_RE: Lazy<Regex> = Lazy::new(|| Regex::new("(!?)\\[[^\\]]*\\]\\(([^)]*)\\)").unwrap());
static A_HTML_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<a href="(.*)">(.*)</a>"#).unwrap());
static IMG_HTML_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<img src="([^"]*)"[^>]*>"#).unwrap());
static SOURCE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\[(\d+)\]"#).unwrap());
static CODE_RE: Lazy<Regex> = Lazy::new(|| Regex::new("`[^`]+`").unwrap());

impl Line {
    pub fn from<S: Into<String>>(text: S) -> Line {
        Line(text.into())
    }

    /// provides all links and images in this line
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

    /// provides the indexes of all sources referenced on this line
    pub fn source_references(&self) -> Vec<SourceReference> {
        let sanitized = CODE_RE.replace_all(&self.0, "");
        let mut result = vec![];
        for captures in SOURCE_RE.captures_iter(&sanitized) {
            let total_match = captures.get(0).unwrap();
            result.push(SourceReference {
                identifier: captures.get(1).unwrap().as_str().to_string(),
                start: total_match.start() as u32,
                end: total_match.end() as u32,
            });
        }
        result
    }
}

#[cfg(test)]
mod tests {

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

    mod used_sources {
        use crate::database::{Line, SourceReference};

        #[test]
        fn no_source() {
            let line = Line::from("text");
            let have = line.source_references();
            assert_eq!(have.len(), 0);
        }

        #[test]
        fn single_source() {
            let line = Line::from("- text [1]");
            let have = line.source_references();
            let want = vec![SourceReference {
                identifier: "1".into(),
                start: 7,
                end: 10,
            }];
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn multiple_sources() {
            let line = Line::from("- text [1] [2]");
            let have = line.source_references();
            let want = vec![
                SourceReference {
                    identifier: "1".into(),
                    start: 7,
                    end: 10,
                },
                SourceReference {
                    identifier: "2".into(),
                    start: 11,
                    end: 14,
                },
            ];
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn ignore_code_looking_like_source_references() {
            let line = Line::from("code: `map[0]`");
            let have = line.source_references();
            pretty::assert_eq!(have, vec![]);
        }
    }
}
