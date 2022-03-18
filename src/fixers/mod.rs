mod empty_section;
mod missing_link;
mod obsolete_link;
mod unordered_sections;

use crate::config;
use crate::database::Tikibase;
use crate::issue::Issue;
use empty_section::remove_empty_section;
use missing_link::add_missing_links;
use obsolete_link::remove_obsolete_links;
use unordered_sections::sort_unordered_sections;

pub fn fix(issue: &Issue, base: &mut Tikibase, config: &config::Data) -> Option<String> {
    match issue {
        Issue::BrokenImage {
            filename: _,
            line: _,
            target: _,
        } => None,
        Issue::BrokenLink {
            filename: _,
            line: _,
            target: _,
        } => None,
        Issue::DuplicateSection {
            filename: _,
            section_type: _,
        } => None,
        Issue::EmptySection {
            filename,
            line,
            section_type,
        } => Some(remove_empty_section(
            base,
            section_type,
            filename,
            line.clone(),
        )),
        Issue::LinkToSameDocument {
            filename: _,
            line: _,
        } => None,
        Issue::LinkWithoutDestination {
            filename: _,
            line: _,
        } => None,
        Issue::MissingLinks { file, links } => Some(add_missing_links(base, file, links)),
        Issue::MissingSource {
            file: _,
            line: _,
            index: _,
        } => None,
        Issue::MixCapSection { variants: _ } => None,
        Issue::ObsoleteLink { file, line } => Some(remove_obsolete_links(base, file, line.clone())),
        Issue::OrphanedResource { path: _ } => None,
        Issue::SectionWithoutHeader { file: _, line: _ } => None,
        Issue::UnknownSection {
            file: _,
            line: _,
            section_type: _,
            allowed_types: _,
        } => None,
        Issue::UnorderedSections { file } => Some(sort_unordered_sections(
            base,
            file,
            &config.sections.unwrap(),
        )),
    }
}
