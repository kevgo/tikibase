use crate::core::tikibase::Tikibase;
use heck::KebabCase;

/// provides all valid link targets for the given Tikibase
pub fn find(base: &Tikibase) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for doc in &base.docs {
        let filename = doc.relative_path(&base.dir);
        result.push(filename.clone());
        result.push(format!(
            "{}{}",
            &filename,
            make_anchor(&doc.title_section.title_line)
        ));
        for section in &doc.content_sections {
            result.push(format!("{}{}", &filename, make_anchor(&section.title_line)));
        }
    }
    result.sort();
    result
}

/// converts the given link title into a GitHub-compatible link anchor
fn make_anchor(title: &str) -> String {
    format!("#{}", title.to_kebab_case())
}

#[cfg(test)]
mod tests {
    use crate::core::persistence;
    use std::path::PathBuf;

    #[test]
    fn make_anchor() {
        let tests = vec![("foo", "#foo")];
        for (give, want) in tests.into_iter() {
            assert_eq!(super::make_anchor(give), want);
        }
    }

    #[test]
    fn find_link_targets() {
        let content = "\
# One

### Alpha
### Beta

content";
        let mut base = persistence::tmpbase();
        base.create_doc(&PathBuf::from("one.md"), content);
        base.create_doc(&PathBuf::from("two.md"), content);
        let have = super::find(&base);
        let want = vec![
            "one.md",
            "one.md#alpha",
            "one.md#beta",
            "one.md#one",
            "two.md",
            "two.md#alpha",
            "two.md#beta",
            "two.md#one",
        ];
        assert_eq!(have, want);
    }
}
