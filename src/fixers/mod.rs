use crate::config;
use crate::database::Tikibase;

pub(crate) mod empty_section;
pub(crate) mod missing_link;
pub(crate) mod obsolete_link;
pub(crate) mod unordered_sections;

pub(crate) trait Fix {
    /// fixes the associated issue, returns a human-readable description of what it did
    fn fix(&self, base: &mut Tikibase, config: &config::Data) -> String;
}
