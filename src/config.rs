use serde::Deserialize;
// use serde_json::Result;
use std::fs::File;
use std::io::ErrorKind;

/// Tikibase configuration data
#[derive(Deserialize, Default)]
pub struct Data {
    /// the allowed section types
    pub allowed_sections: Option<Vec<String>>,
}

/// loads the configuration from the config file
pub fn load() -> Result<Data, String> {
    let file = match File::open("tikibase.json") {
        Ok(content) => content,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Ok(Default::default()),
            _ => {
                return Err(format!(
                    "Cannot access configuration file (tikibase.json): {}",
                    e
                ))
            }
        },
    };
    serde_json::from_reader(file)
        .map_err(|e| format!("Configuration file has invalid structure: {}", e))
}

#[cfg(test)]
mod tests {

    use super::Data;

    #[test]
    fn defaults() {
        let default_config: Data = Default::default();
        assert_eq!(default_config.allowed_sections, None)
    }
}
