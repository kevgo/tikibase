use super::scanners::{section_capitalization, section_level};
use ahash::AHashMap;

pub struct State2 {
    pub capitalization_outliers: AHashMap<String, section_capitalization::OutlierInfo>,
    pub level_outliers: AHashMap<String, section_level::OutlierInfo>,
}
