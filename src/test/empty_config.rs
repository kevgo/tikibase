use crate::config;

/// provides an empty configuration instance for testing
pub fn empty_config() -> Config {
    Config {
        sections: None,
        ignore: None,
    }
}
