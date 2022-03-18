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
            filename,
            line,
            target,
        } => None,
        Issue::BrokenLink {
            filename,
            line,
            target,
        } => None,
        Issue::DuplicateSection {
            filename,
            section_type,
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
        Issue::LinkToSameDocument { filename, line } => None,
        Issue::LinkWithoutDestination { filename, line } => None,
        Issue::MissingLinks { file, links } => Some(add_missing_links(base, file, links)),
        Issue::MissingSource { file, line, index } => None,
        Issue::MixCapSection { variants } => None,
        Issue::ObsoleteLink { file, line } => Some(remove_obsolete_links(base, file, line.clone())),
        Issue::OrphanedResource { path } => None,
        Issue::SectionWithoutHeader { file, line } => None,
        Issue::UnknownSection {
            file,
            line,
            section_type,
            allowed_types,
        } => None,
        Issue::UnorderedSections { file } => Some(sort_unordered_sections(
            base,
            file,
            &config.sections.unwrap(),
        )),
    }
}
