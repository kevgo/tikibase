use crate::Issue;
use std::fmt::{self, Display, Formatter};

/// describes the issue that sections have mixed capitalization
pub struct MixCapSection {
    pub variants: Vec<String>,
}

impl Display for MixCapSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "mixed capitalization of sections: {}",
            self.variants.join("|")
        )
    }
}

impl Issue for MixCapSection {
    fn fixable(&self) -> bool {
        false
    }
}
