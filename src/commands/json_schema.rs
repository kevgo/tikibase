use crate::{config, Issue, Location, Outcome};
use colored::Colorize;
use schemars::schema_for;
use std::fs::File;

/// exports the JSON Schema for the configuration file
pub fn json_schema() -> Outcome {
    let filename = "tikibase.schema.json";
    print!("exporting {} ... ", filename);
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
    let schema = schema_for!(config::Config);
    serde_json::to_writer_pretty(file, &schema).unwrap();
    println!("{}", "ok".green());
    Outcome::default()
}
