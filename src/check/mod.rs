use crate::core::tikibase::Tikibase;
mod empty_sections;
mod section_capitalization;

pub fn run() {
    let base = Tikibase::in_dir(".");
    for error in section_capitalization::check(&base) {
        println!(
            "- mixed capitalization of sections: \"{}\"",
            error.variants.join("\", \"")
        );
    }
    for empty_section in empty_sections::find(&base) {
        println!(
            "- {}:{} empty section",
            empty_section.path.to_str().unwrap(),
            empty_section.line
        );
    }
}
