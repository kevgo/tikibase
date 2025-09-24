use crate::config;
use crate::prelude::*;
use colored::Colorize;
use fs_err::File;
use schemars::schema_for;

/// exports the JSON Schema for the configuration file
pub fn json_schema() -> Result<()> {
  let filename = "tikibase.schema.json";
  print!("exporting {filename} ... ");
  let file = File::create(filename).map_err(|err| UserError::CannotWriteFile {
    filename: "tikibase.schema.json".into(),
    reason: err.to_string(),
  })?;
  let schema = schema_for!(config::Config);
  serde_json::to_writer_pretty(file, &schema).unwrap();
  println!("{}", "ok".green());
  Ok(())
}
