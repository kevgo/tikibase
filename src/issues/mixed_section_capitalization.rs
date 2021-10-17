use crate::Issue;

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

    fn fixable(&self) -> bool {
        false
    }
}
