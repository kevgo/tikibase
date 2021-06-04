use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Line {
    pub text: String,
}

pub enum Reference {
    Link { destination: String },
    Image { src: String },
}

impl Line {
    /// provides all links and images in this line
    pub fn references(&self) -> Vec<Reference> {
        lazy_static! {
            static ref MD_RE: Regex = Regex::new("(!?)\\[[^\\]]*\\]\\(([^)]*)\\)").unwrap();
            static ref A_HTML_RE: Regex = Regex::new(r#"<a href="(.*)">(.*)</a>"#).unwrap();
            static ref IMG_HTML_RE: Regex = Regex::new(r#"<img src="([^"]*)"[^>]*>"#).unwrap();
        }
        let mut result = Vec::new();
        for cap in MD_RE.captures_iter(&self.text) {
            match &cap[1] {
                "!" => result.push(Reference::Image {
                    src: cap[2].to_string(),
                }),
                "" => {
                    let mut destination = cap[2].to_string();
                    if let Some(idx) = destination.find('#') {
                        destination.truncate(idx);
                    }
                    result.push(Reference::Link { destination })
                }
                _ => panic!("unexpected capture: '{}'", &cap[1]),
            }
        }
        for cap in A_HTML_RE.captures_iter(&self.text) {
            result.push(Reference::Link {
                destination: cap[1].to_string(),
            });
        }
        for cap in IMG_HTML_RE.captures_iter(&self.text) {
            result.push(Reference::Image {
                src: cap[1].to_string(),
            });
        }
        result
    }

    /// provides the indexes of all sources used in this line
    pub fn used_sources(&self) -> Vec<String> {
        lazy_static! {
            static ref SOURCE_RE: Regex = Regex::new(r#"\[(\d+)\]"#).unwrap();
            static ref CODE_RE: Regex = Regex::new("`[^`]+`").unwrap();
        }
        let sanitized = CODE_RE.replace_all(&self.text, "");
        SOURCE_RE
            .captures_iter(&sanitized)
            .map(|cap| cap[1].to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {

    mod references {
        use crate::testhelpers::line_with_text;

        use super::super::*;
        #[test]
        fn link_md() {
            let line = line_with_text(
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
            let line = line_with_text(r#"an HTML link: <a href="two.md">two</a>"#);
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
            let line = line_with_text(r#"an MD image: ![zonk](zonk.md)"#);
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
            let line = line_with_text(r#"<img src="zonk.md">"#);
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
            let line = line_with_text(r#"<img src="zonk.md" width="10" height="10">"#);
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
            let line = line_with_text(r#"<img src="zonk.md"/>"#);
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
            let line = line_with_text(r#"<img src="zonk.md" />"#);
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
        use crate::testhelpers::line_with_text;

        #[test]
        fn no_source() {
            let line = line_with_text("text");
            let have = line.used_sources();
            assert_eq!(have.len(), 0);
        }

        #[test]
        fn single_source() {
            let line = line_with_text("- text [1]");
            let have = line.used_sources();
            assert_eq!(have, vec!["1".to_string()]);
        }

        #[test]
        fn multiple_sources() {
            let line = line_with_text("- text [1] [2]");
            let have = line.used_sources();
            assert_eq!(have, vec!["1".to_string(), "2".to_string()]);
        }

        #[test]
        fn code_segment() {
            let line = line_with_text("code: `map[0]`");
            let have = line.used_sources();
            let want: Vec<String> = Vec::new();
            assert_eq!(have, want);
        }
    }
}
