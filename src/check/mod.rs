use crate::core::tikibase::Tikibase;
mod empty_sections;
mod section_capitalization;

enum Error {
    MixedCapitalization { variants: Vec<String> },
    EmptySection { filename: String, line: u32 },
}

pub fn run() {
    let base = Tikibase::in_dir(".");
    for error in section_capitalization::check(&base) {
        println!(
            "- mixed capitalization of sections: \"{}\"",
            error.variants.join("\", \"")
        );
    }
    for section in empty_sections::find(&base) {
        println!(
            "- {}:{} empty section",
            section.path.to_str().unwrap(),
            section.line_number
        );
    }
}
