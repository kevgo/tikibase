#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Link {
    pub target: String,
    pub line: u32,
    pub start: u32,
    pub end: u32,
}

impl Link {
    pub fn points_to(&self, path: &str) -> bool {
        match self.target.split_once('#') {
            Some((base, _anchor)) => base == path,
            None => self.target == path,
        }
    }
}

#[cfg(test)]
mod tests {

    mod points_to {
        use crate::database::Link;
        use big_s::S;

        #[test]
        fn matching_link() {
            let img = Link {
                target: S("ok.md"),
                line: 0,
                start: 0,
                end: 0,
            };
            assert!(img.points_to("ok.md"));
        }

        #[test]
        fn mismatching_link() {
            let img = Link {
                target: S("ok.md"),
                line: 0,
                start: 0,
                end: 0,
            };
            assert!(!img.points_to("other.md"));
        }

        #[test]
        fn with_anchor() {
            let img = Link {
                target: S("ok.md#foo"),
                line: 0,
                start: 0,
                end: 0,
            };
            assert!(img.points_to("ok.md"));
        }
    }
}
