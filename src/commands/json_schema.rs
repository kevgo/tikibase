use crate::{config, Issue, Location, Outcome};
use schemars::schema_for;
use std::fs::File;

/// exports the JSON Schema for the configuration file
pub fn json_schema() -> Outcome {
    let schema = schema_for!(config::Config);
    let filename = "tikibase.schema.json";
    print!("saving {} ... ", filename);
    let file = match File::create(filename) {
        Ok(file) => file,
        Err(err) => {
            return Outcome::from_issue(Issue::CannotWriteJsonSchemaFile {
                location: Location {
                    file: filename.into(),
                    line: 0,
                    start: 0,
                    end: 0,
                },
                message: err.to_string(),
            })
        }
    };
    serde_json::to_writer_pretty(file, &schema).unwrap();
    Outcome::default()
}
