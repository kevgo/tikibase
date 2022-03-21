use super::Reference;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Line(String);

static MD_RE: Lazy<Regex> = Lazy::new(|| Regex::new("(!?)\\[[^\\]]*\\]\\(([^)]*)\\)").unwrap());
static A_HTML_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<a href="(.*)">(.*)</a>"#).unwrap());
static IMG_HTML_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"<img src="([^"]*)"[^>]*>"#).unwrap());
static SOURCE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"\[(\d+)\]"#).unwrap());
static CODE_RE: Lazy<Regex> = Lazy::new(|| Regex::new("`[^`]+`").unwrap());

impl Line {
    // TODO: rename to from
    pub fn from<S: Into<String>>(text: S) -> Line {
        Line(text.into())
    }

    /// provides all links and images in this line
    pub fn references(&self) -> Vec<Reference> {
        let mut result = Vec::new();
        for cap in MD_RE.captures_iter(&self.0) {
            match &cap[1] {
                "!" => result.push(Reference::Image {
                    src: cap[2].to_string(),
                }),
                "" => {
                    let mut destination = cap[2].to_string();
                    if let Some(idx) = destination.find('#') {
                        destination.truncate(idx);
                    }
                    result.push(Reference::Link { destination });
                }
                _ => panic!("unexpected capture: '{}'", &cap[1]),
            }
        }
        for cap in A_HTML_RE.captures_iter(&self.0) {
            result.push(Reference::Link {
                destination: cap[1].to_string(),
            });
        }
        for cap in IMG_HTML_RE.captures_iter(&self.0) {
            result.push(Reference::Image {
                src: cap[1].to_string(),
            });
        }
        result
    }

    /// provides the text of this line
    pub fn text(&self) -> &str {
        &self.0
    }

    /// provides the indexes of all sources used in this line
    pub fn used_sources(&self) -> Vec<String> {
        let sanitized = CODE_RE.replace_all(&self.0, "");
        SOURCE_RE
            .captures_iter(&sanitized)
            .map(|cap| cap[1].to_string())
            .collect()
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
                },
                Reference::Link {
                    destination: "two.md".into(),
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
            }];
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn img_md() {
            let line = Line::from(r#"an MD image: ![zonk](zonk.md)"#);
            let have = line.references();
            let want = vec![Reference::Image {
                src: "zonk.md".into(),
            }];
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn img_html() {
            let line = Line::from(r#"<img src="zonk.md">"#);
            let have = line.references();
            let want = vec![Reference::Image {
                src: "zonk.md".into(),
            }];
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn img_html_extra_attributes() {
            let line = Line::from(r#"<img src="zonk.md" width="10" height="10">"#);
            let have = line.references();
            let want = vec![Reference::Image {
                src: "zonk.md".into(),
            }];
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn img_xml_nospace() {
            let line = Line::from(r#"<img src="zonk.md"/>"#);
            let have = line.references();
            let want = vec![Reference::Image {
                src: "zonk.md".into(),
            }];
            pretty::assert_eq!(have, want)
        }

        #[test]
        fn img_xml_space() {
            let line = Line::from(r#"<img src="zonk.md" />"#);
            let have = line.references();
            let want = vec![Reference::Image {
                src: "zonk.md".into(),
            }];
            pretty::assert_eq!(have, want)
        }
    }

    mod used_sources {
        use crate::database::Line;

        #[test]
        fn no_source() {
            let line = Line::from("text");
            let have = line.used_sources();
            assert_eq!(have.len(), 0);
        }

        #[test]
        fn single_source() {
            let line = Line::from("- text [1]");
            let have = line.used_sources();
            let want = vec!["1".to_string()];
            assert_eq!(have, want);
        }

        #[test]
        fn multiple_sources() {
            let line = Line::from("- text [1] [2]");
            let have = line.used_sources();
            let want = vec!["1".to_string(), "2".to_string()];
            assert_eq!(have, want);
        }

        #[test]
        fn code_segment() {
            let line = Line::from("code: `map[0]`");
            let have = line.used_sources();
            let want: Vec<String> = vec![];
            assert_eq!(have, want);
        }
    }
}
