use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Line {
    /// The line number relative to the section title line, 0-based.
    pub section_offset: u32,
    pub text: String,
}

pub enum Reference {
    Link { destination: String, title: String },
    Image { src: String },
}

impl Line {
    pub fn references(&self) -> Vec<Reference> {
        lazy_static! {
            static ref MD_RE: Regex = Regex::new("(!?)\\[(.*)\\]\\((.*)\\)").unwrap();
            static ref A_HTML_RE: Regex = Regex::new(r#"<a href="(.*)">(.*)</a>"#).unwrap();
            static ref IMG_HTML_RE: Regex = Regex::new(r#"<img src="(.*)"\s*/?>"#).unwrap();
        }
        let mut result = Vec::new();
        for cap in MD_RE.captures_iter(&self.text) {
            match &cap[1] {
                "!" => result.push(Reference::Image {
                    src: cap[3].to_string(),
                }),
                "" => result.push(Reference::Link {
                    title: cap[2].to_string(),
                    destination: cap[3].to_string(),
                }),
                _ => panic!("unexpected capture: '{}'", &cap[1]),
            }
        }
        for cap in A_HTML_RE.captures_iter(&self.text) {
            result.push(Reference::Link {
                title: cap[2].to_string(),
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
}

#[cfg(test)]
mod tests {

    mod references {
        use super::super::*;
        #[test]
        fn link_md() {
            let line = Line {
                section_offset: 0,
                text: r#"an MD link: [one](one.md)"#.to_string(),
            };
            let have = line.references();
            assert_eq!(have.len(), 1);
            match &have[0] {
                Reference::Link { destination, title } => {
                    assert_eq!(title, "one");
                    assert_eq!(destination, "one.md");
                }
                Reference::Image { src: _ } => panic!("unexpected image"),
            };
        }

        #[test]
        fn link_html() {
            let line = Line {
                section_offset: 0,
                text: r#"an HTML link: <a href="two.md">two</a>"#.to_string(),
            };
            let have = line.references();
            assert_eq!(have.len(), 1);
            match &have[0] {
                Reference::Link { destination, title } => {
                    assert_eq!(title, "two");
                    assert_eq!(destination, "two.md");
                }
                Reference::Image { src: _ } => panic!("unexpected image"),
            };
        }

        #[test]
        fn img_md() {
            let line = Line {
                section_offset: 0,
                text: r#"an MD image: ![zonk](zonk.md)"#.to_string(),
            };
            let have = line.references();
            assert_eq!(have.len(), 1);
            match &have[0] {
                Reference::Link {
                    destination: _,
                    title: _,
                } => panic!("unexpected link"),
                Reference::Image { src } => {
                    assert_eq!(src, "zonk.md");
                }
            };
        }

        #[test]
        fn img_html() {
            let line = Line {
                section_offset: 0,
                text: r#"<img src="zonk.md">"#.to_string(),
            };
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
            let line = Line {
                section_offset: 0,
                text: r#"<img src="zonk.md"/>"#.to_string(),
            };
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
            let line = Line {
                section_offset: 0,
                text: r#"<img src="zonk.md" />"#.to_string(),
            };
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
}
