use crate::checks::Issue;
use crate::config;
use crate::database::Tikibase;

/// describes the issue that sections have mixed capitalization
pub struct MixCapSection {
    pub variants: Vec<String>,
}

impl Issue for MixCapSection {
    fn describe(&self) -> String {
        format!(
            "mixed capitalization of sections: {}",
            self.variants.join("|")
        )
    }

    fn fix(&self, _base: &mut Tikibase, _config: &config::Data) -> String {
        panic!("not fixable")
    }

    fn fixable(&self) -> bool {
        false
    }
}
