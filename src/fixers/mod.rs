use crate::config;
use crate::database::Tikibase;

pub(crate) mod empty_section;
pub(crate) mod missing_link;

pub(crate) trait Fix {
    /// fixes the associated issue, returns a human-readable description of what it did
    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String;
}
