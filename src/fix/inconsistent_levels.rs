use super::Fix::NormalizedSectionLevel;
use crate::database::Tikibase;
use crate::fix::Result::Fixed;
use crate::{fix, Location};

pub fn normalize_outliers(base: &mut Tikibase) -> fix::Result {
    Fixed(NormalizedSectionLevel {
        location: Location::default(),
    })
}
