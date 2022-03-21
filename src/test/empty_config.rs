use crate::config;

/// provides an empty configuration instance for testing
pub fn empty_config() -> config::Data {
    config::Data {
        sections: None,
        ignore: None,
    }
}
