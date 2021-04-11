use crate::core::tikibase::Tikibase;
mod checker;
mod empty_sections;
mod section_capitalization;

pub fn run() {
    let base = Tikibase::in_dir(".");
    let mut localized_issues = checker::LocalizedIssueCollector::new();
    localized_issues.register(empty_sections::find(&base));
    for (location, issue) in localized_issues.issues {
        println!("{} - {}\n", location, issue.desc());
    }
    for error in section_capitalization::check(&base) {
        println!(
            "- mixed capitalization of sections: \"{}\"",
            error.variants.join("\", \"")
        );
    }
}
