use super::Fix::NormalizedSectionCapitalization;
use crate::database::Tikibase;
use crate::fix::Result::Fixed;
use crate::{fix, Location};

pub fn normalize_capitalization(
    base: &mut Tikibase,
    location: Location,
    section_level: u8,
    old_capitalization: String,
    new_capitalization: String,
) -> fix::Result {
    let base_dir = base.dir.clone();
    let doc = base.get_doc_mut(&location.file).unwrap();
    let section = doc.section_with_title_mut(&old_capitalization).unwrap();
    section.title_line.text = title_at_level(&new_capitalization, section_level as usize);
    doc.save(&base_dir);
    Fixed(NormalizedSectionCapitalization {
        location,
        old_capitalization,
        new_capitalization,
    })
}

fn title_at_level(title: &str, level: usize) -> String {
    format!("{} {}", "#".repeat(level), title)
}

#[cfg(test)]
mod tests {

    mod title_at_level {
        use super::super::title_at_level;

        #[test]
        fn one() {
            let have = title_at_level("title", 1);
            let want = "# title".to_string();
            assert_eq!(have, want);
        }

        #[test]
        fn six() {
            let have = title_at_level("title", 6);
            let want = "###### title".to_string();
            assert_eq!(have, want);
        }
    }
}
