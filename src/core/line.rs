use super::link::Link;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Line {
    /// The line number relative to the section title line, 0-based.
    pub section_offset: u32,
    pub text: String,
}

impl Line {
    pub fn links(&self) -> Vec<Link> {
        lazy_static! {
            static ref MD_LINK_RE: Regex = Regex::new("\\[(.*)\\]\\((.*)\\)").unwrap();
            static ref HTML_LINK_RE: Regex = Regex::new(r#"<a href="(.*)">(.*)</a>"#).unwrap();
        }
        let mut result = Vec::new();
        for cap in MD_LINK_RE.captures_iter(&self.text) {
            result.push(Link {
                title: cap[1].to_string(),
                destination: cap[2].to_string(),
            });
        }
        for cap in HTML_LINK_RE.captures_iter(&self.text) {
            result.push(Link {
                title: cap[2].to_string(),
                destination: cap[1].to_string(),
            });
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links() {
        let line = Line {
            section_offset: 0,
            text: r#"[one](one.md) and <a href="two.md">two</a>"#.to_string(),
        };
        let links = line.links();
        assert_eq!(links.len(), 2);
        assert_eq!(
            links[0],
            Link {
                title: "one".to_string(),
                destination: "one.md".to_string()
            }
        );
        assert_eq!(
            links[1],
            Link {
                title: "two".to_string(),
                destination: "two.md".to_string()
            }
        );
    }
}
