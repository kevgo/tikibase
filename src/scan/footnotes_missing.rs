use crate::database::{FootnoteDefinition, FootnoteReference};
use crate::{Issue, Location, Tikibase};

pub(crate) fn scan(base: &Tikibase) -> Vec<Issue> {
    let mut issues = Vec::<Issue>::new();
    for doc in &base.docs {
        let footnote_references = match doc.footnote_references() {
            Ok(used_sources) => used_sources,
            Err(issue) => return vec![issue],
        };
        let footnote_definitions = match doc.footnote_definitions() {
            Ok(footnote_definitions) => footnote_definitions,
            Err(issue) => return vec![issue],
        };
        for footnote_reference in &footnote_references {
            if !contains_reference(&footnote_definitions, &footnote_reference.identifier) {
                issues.push(Issue::MissingSource {
                    location: Location {
                        file: doc.path.clone(),
                        line: footnote_reference.line,
                        start: footnote_reference.start,
                        end: footnote_reference.end,
                    },
                    identifier: footnote_reference.identifier.clone(),
                });
            }
        }
        for footnote_definition in &footnote_definitions {
            if !contains_definition(&footnote_references, &footnote_definition.identifier) {
                issues.push(Issue::UnusedFootnote {
                    location: Location {
                        file: doc.path.clone(),
                        line: footnote_definition.line,
                        start: footnote_definition.start,
                        end: footnote_definition.end,
                    },
                    identifier: footnote_definition.identifier.clone(),
                })
            }
        }
    }
    issues
}

fn contains_reference(definitions: &[FootnoteDefinition], identifier: &str) -> bool {
    definitions
        .iter()
        .any(|definition| definition.identifier == identifier)
}

fn contains_definition(references: &[FootnoteReference], identifier: &str) -> bool {
    references
        .iter()
        .any(|reference| reference.identifier == identifier)
}
