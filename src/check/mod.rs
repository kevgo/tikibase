use crate::core::tikibase::Tikibase;
mod section_capitalization;

pub fn run() {
    let base = Tikibase::in_dir(".");
    for error in section_capitalization::find(&base) {
        println!(
            "- mixed capitalization of sections: \"{}\"",
            error.variants.join("\", \"")
        );
    }
}
