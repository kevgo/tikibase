use super::Outcome;
use crate::check::Issue;
use crate::config;
use colored::Colorize;
use fs_err::File;
use schemars::schema_for;

/// exports the JSON Schema for the configuration file
#[must_use]
pub fn json_schema() -> Outcome {
    let filename = "tikibase.schema.json";
    print!("exporting {filename} ... ");
    let file = match File::create(filename) {
        Ok(file) => file,
        Err(err) => {
            return Outcome::from_issue(Issue::CannotWriteJsonSchemaFile {
                file: filename.into(),
                message: err.to_string(),
            })
        }
    };
    let schema = schema_for!(config::Config);
    serde_json::to_writer_pretty(file, &schema).unwrap();
    println!("{}", "ok".green());
    Outcome::default()
}
