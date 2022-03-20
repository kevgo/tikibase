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
    pub fn new<S: Into<String>>(text: S) -> Line {
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
            let line = Line::new(
                r#"an MD link: [one](one.md) and one to a section: [two pieces](two.md#pieces)!"#,
            );
            let have = line.references();
            assert_eq!(have.len(), 2);
            match &have[0] {
                Reference::Link { destination } => {
                    assert_eq!(destination, "one.md");
                }
                Reference::Image { src: _ } => panic!("unexpected image"),
            };
            match &have[1] {
                Reference::Link { destination } => {
                    assert_eq!(destination, "two.md");
                }
                Reference::Image { src: _ } => panic!("unexpected image"),
            };
        }

        #[test]
        fn link_html() {
            let line = Line::new(r#"an HTML link: <a href="two.md">two</a>"#);
            let have = line.references();
            assert_eq!(have.len(), 1);
            match &have[0] {
                Reference::Link { destination } => {
                    assert_eq!(destination, "two.md");
                }
                Reference::Image { src: _ } => panic!("unexpected image"),
            };
        }

        #[test]
        fn img_md() {
            let line = Line::new(r#"an MD image: ![zonk](zonk.md)"#);
            let have = line.references();
            assert_eq!(have.len(), 1);
            match &have[0] {
                Reference::Link { destination: _ } => panic!("unexpected link"),
                Reference::Image { src } => {
                    assert_eq!(src, "zonk.md");
                }
            };
        }

        #[test]
        fn img_html() {
            let line = Line::new(r#"<img src="zonk.md">"#);
            let have = line.references();
            assert_eq!(have.len(), 1);
            match &have[0] {
                Reference::Image { src } => {
                    assert_eq!(src, "zonk.md");
                }
                _ => panic!("expected image"),
            };
        }

        #[test]
        fn img_html_extra_attributes() {
            let line = Line::new(r#"<img src="zonk.md" width="10" height="10">"#);
            let have = line.references();
            assert_eq!(have.len(), 1);
            match &have[0] {
                Reference::Image { src } => {
                    assert_eq!(src, "zonk.md");
                }
                _ => panic!("expected image"),
            };
        }

        #[test]
        fn img_xml_nospace() {
            let line = Line::new(r#"<img src="zonk.md"/>"#);
            let have = line.references();
            assert_eq!(have.len(), 1);
            match &have[0] {
                Reference::Image { src } => {
                    assert_eq!(src, "zonk.md");
                }
                _ => panic!("expected image"),
            };
        }

        #[test]
        fn img_xml_space() {
            let line = Line::new(r#"<img src="zonk.md" />"#);
            let have = line.references();
            assert_eq!(have.len(), 1);
            match &have[0] {
                Reference::Image { src } => {
                    assert_eq!(src, "zonk.md");
                }
                _ => panic!("expected image"),
            };
        }
    }

    mod used_sources {
        use crate::database::Line;

        #[test]
        fn no_source() {
            let line = Line::new("text");
            let have = line.used_sources();
            assert_eq!(have.len(), 0);
        }

        #[test]
        fn single_source() {
            let line = Line::new("- text [1]");
            let have = line.used_sources();
            assert_eq!(have, vec!["1".to_string()]);
        }

        #[test]
        fn multiple_sources() {
            let line = Line::new("- text [1] [2]");
            let have = line.used_sources();
            assert_eq!(have, vec!["1".to_string(), "2".to_string()]);
        }

        #[test]
        fn code_segment() {
            let line = Line::new("code: `map[0]`");
            let have = line.used_sources();
            let want: Vec<String> = Vec::new();
            assert_eq!(have, want);
        }
    }
}
