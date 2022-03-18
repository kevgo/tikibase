use crate::config;
use crate::database::Tikibase;
use crate::issue::Issue;

use self::empty_section::EmptySectionFixer;
use self::missing_link::MissingLinksFixer;
use self::obsolete_link::ObsoleteLinkFixer;
use self::unordered_sections::UnorderedSectionFixer;

pub(crate) mod empty_section;
pub(crate) mod missing_link;
pub(crate) mod obsolete_link;
pub(crate) mod unordered_sections;

pub(crate) trait Fix {
    /// fixes the associated issue, returns a human-readable description of what it did
    fn fix(&self, base: &mut Tikibase, config: &config::Data) -> String;
}

pub fn fixer(issue: Issue) -> Option<Fix> {
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
        } => Some(Box::new(EmptySectionFixer { issue })),
        Issue::LinkToSameDocument { filename, line } => None,
        Issue::LinkWithoutDestination { filename, line } => None,
        Issue::MissingLinks { file, links } => Some(Box::new(MissingLinksFixer { issue })),
        Issue::MissingSource { file, line, index } => None,
        Issue::MixCapSection { variants } => None,
        Issue::ObsoleteLink { file, line } => Some(Box::new(ObsoleteLinkFixer { issue })),
        Issue::OrphanedResource { path } => None,
        Issue::SectionWithoutHeader { file, line } => None,
        Issue::UnknownSection {
            file,
            line,
            section_type,
            allowed_types,
        } => None,
        Issue::UnorderedSections { file } => Some(Box::new(UnorderedSectionFixer { issue })),
    }
}
