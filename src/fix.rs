use crate::Message;
use serde::Serialize;
use std::path::PathBuf;

/// documents the fixes that this linter performs
#[derive(Serialize)]
pub enum Fix {
    AddedOccurrencesSection {
        file: PathBuf,
        line: u32,
    },
    RemovedEmptySection {
        section_type: String,
        file: PathBuf,
        line: u32,
    },
    RemovedObsoleteOccurrencesSection {
        file: PathBuf,
        line: u32,
    },
    SortedSections {
        file: PathBuf,
    },
}

impl Fix {
    pub fn to_message(self) -> Message {
        match self {
            Fix::RemovedEmptySection {
                section_type,
                file,
                line,
            } => Message {
                text: format!("removed empty section \"{}\"", section_type),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Fix::AddedOccurrencesSection { file, line } => Message {
                text: "added occurrences section".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Fix::RemovedObsoleteOccurrencesSection { file, line } => Message {
                text: "removed obsolete occurrences section".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: Some(line),
            },
            Fix::SortedSections { file } => Message {
                text: "fixed section order".into(),
                file: Some(file.to_string_lossy().to_string()),
                line: None,
            },
        }
    }
}
