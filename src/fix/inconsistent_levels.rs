use super::Fix::NormalizedSectionLevel;
use crate::check::Location;
use crate::database::Tikibase;
use crate::fix;
use crate::fix::Result::Fixed;

pub fn normalize_outliers(
    base: &mut Tikibase,
    location: Location,
    section_human_title: String,
    old_level: u8,
    new_level: u8,
) -> fix::Result {
    let base_dir = base.root.clone();
    let doc = base.get_doc_mut(&location.file).unwrap();
    let section = doc
        .section_with_human_title_mut(&section_human_title)
        .unwrap();
    section.title_line.text = title_at_level(&section_human_title, new_level as usize);
    doc.save(&base_dir);
    Fixed(NormalizedSectionLevel {
        location,
        section_human_title,
        old_level,
        new_level,
    })
}

pub fn set_to_configured_section_level(
    base: &mut Tikibase,
    location: Location,
    actual_level: u8,
    actual_title: String,
    configured_level: u8,
    configured_title: String,
) -> fix::Result {
    let base_dir = base.root.clone();
    let doc = base.get_doc_mut(&location.file).unwrap();
    let section = doc.section_with_title_mut(&actual_title).unwrap();
    section.title_line.text = configured_title;
    doc.save(&base_dir);
    Fixed(NormalizedSectionLevel {
        location,
        section_human_title: actual_title,
        old_level: actual_level,
        new_level: configured_level,
    })
}

pub fn title_at_level(title: &str, level: usize) -> String {
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
