/// a link in the document
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Reference {
    Link {
        target: String,
        line: u32,
        start: u32,
        end: u32,
    },
    Image {
        src: String,
        line: u32,
        start: u32,
        end: u32,
    },
}

impl Reference {
    /// indicates whether this Reference instance has the given path as its target or src
    pub fn points_to<P: AsRef<str>>(&self, path: P) -> bool {
        match self {
            Reference::Link {
                target,
                line: _,
                start: _,
                end: _,
            } => target == path.as_ref(),
            Reference::Image {
                src,
                line: _,
                start: _,
                end: _,
            } => src == path.as_ref(),
        }
    }
}

#[cfg(test)]
mod tests {

    mod points_to {
        use crate::database::Reference;

        #[test]
        fn matching_image() {
            let img = Reference::Image {
                src: "ok.md".into(),
                line: 0,
                start: 0,
                end: 0,
            };
            assert!(img.points_to("ok.md"));
        }

        #[test]
        fn matching_link() {
            let img = Reference::Link {
                target: "ok.md".into(),
                line: 0,
                start: 0,
                end: 0,
            };
            assert!(img.points_to("ok.md"));
        }

        #[test]
        fn mismatching_image() {
            let img = Reference::Image {
                src: "ok.md".into(),
                line: 0,
                start: 0,
                end: 0,
            };
            assert!(!img.points_to("other.md"));
        }

        #[test]
        fn mismatching_link() {
            let img = Reference::Link {
                target: "ok.md".into(),
                line: 0,
                start: 0,
                end: 0,
            };
            assert!(!img.points_to("other.md"));
        }
    }
}
