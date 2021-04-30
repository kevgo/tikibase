use super::Tikibase;
use crate::core::error::{Outcome, Outcomes};
use crate::core::line::Reference;
use std::path::PathBuf;

pub struct LinksResult {
    pub outcomes: Outcomes,
    /// all resources that are linked to from the given Tikibase
    pub resource_links: Vec<String>,
}

pub fn process(base: &Tikibase) -> LinksResult {
    let mut result = LinksResult {
        outcomes: Outcomes::new(),
        resource_links: Vec::new(),
    };
    let existing_targets = base.link_targets();
    for doc in &base.docs {
        for section in doc.sections() {
            for line in section.lines() {
                for reference in line.references() {
                    match reference {
                        Reference::Link { destination } => {
                            if !destination.starts_with("http")
                                && !existing_targets.contains(&destination)
                            {
                                result.outcomes.push(Outcome::UserError(format!(
                                    "{}:{}  broken link to \"{}\"",
                                    &doc.path.to_string_lossy(),
                                    section.line_number + line.section_offset + 1,
                                    destination,
                                )));
                            }
                        }
                        Reference::Image { src } => {
                            if src.starts_with("http") {
                                continue;
                            }
                            if !base.has_resource(PathBuf::from(&src)) {
                                result.outcomes.push(Outcome::UserError(format!(
                                    "{}:{}  broken image \"{}\"",
                                    &doc.path.to_string_lossy(),
                                    section.line_number + line.section_offset + 1,
                                    &src,
                                )));
                            }
                            result.resource_links.push(src);
                        }
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {

    mod process {
        use crate::core::error::{Outcome, Outcomes};
        use crate::core::tikibase::Tikibase;
        use crate::testhelpers;

        #[test]
        fn link_to_non_existing_file() {
            let dir = testhelpers::tmp_dir();
            let content = "\
# One

[invalid](non-existing.md)
[valid](two.md)
";
            testhelpers::create_file("one.md", content, &dir);
            testhelpers::create_file("two.md", "# Two", &dir);
            let (base, outcomes) = Tikibase::load(dir);
            assert_eq!(outcomes.len(), 0);
            let have = super::super::process(&base);
            assert_eq!(have.outcomes.len(), 1);
            match have.outcomes[0] {
                Outcome::UserError(e) => {
                    assert_eq!(e, "one.md:3  broken link to \"non-existing.md\"")
                }
                Outcome::Notification(_) => panic!("unexpected notification"),
            };
        }

        #[test]
        fn ignore_external_urls() {
            let dir = testhelpers::tmp_dir();
            let content = "\
# One

[external site](https://google.com)
![external image](https://google.com/foo.png)
";
            testhelpers::create_file("one.md", content, &dir);
            testhelpers::create_file("two.md", "# Two", &dir);
            let (base, outcomes) = Tikibase::load(dir);
            assert_eq!(outcomes.len(), 0);
            let have = super::super::process(&base);
            assert_eq!(have.outcomes.len(), 0);
        }

        #[test]
        fn link_to_existing_image() {
            let dir = testhelpers::tmp_dir();
            let content = "\
# One

![image](foo.png)
";
            testhelpers::create_file("1.md", content, &dir);
            testhelpers::create_file("foo.png", "image content", &dir);
            let (base, outcome) = Tikibase::load(dir);
            let have = super::super::process(&base);
            assert_eq!(have.outcomes.len(), 0);
            assert_eq!(have.resource_links.len(), 1);
            assert_eq!(have.resource_links[0], "foo.png");
        }

        #[test]
        fn link_to_non_existing_image() {
            let dir = testhelpers::tmp_dir();
            let content = "\
# One

![image](zonk.png)
";
            testhelpers::create_file("1.md", content, &dir);
            let (base, outcomes) = Tikibase::load(dir);
            assert_eq!(outcomes.len(), 0);
            let have = super::super::process(&base);
            assert_eq!(have.outcomes.len(), 1);
            match have.outcomes[0] {
                Outcome::UserError(e) => assert_eq!(e, "1.md:3  broken image \"zonk.png\""),
                Outcome::Notification(_) => panic!(),
            }
            assert_eq!(have.resource_links.len(), 1);
            assert_eq!(have.resource_links[0], "zonk.png");
        }
    }
}
