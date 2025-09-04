use super::Outcome;
use crate::Fix;
use crate::Tikibase;

#[must_use]
pub fn search(base: &Tikibase, terms: &[String]) -> Outcome {
  if terms.is_empty() {
    println!("No search terms provided");
    return Outcome {
      issues: vec![],
      fixes: vec![Fix::SearchResult {
        text: "No search terms provided".to_string(),
      }],
    };
  }

  let terms_lowercase: Vec<String> = terms.iter().map(|t| t.to_lowercase()).collect();
  let mut all_results = vec![];

  // Sort paths to ensure deterministic output order
  let mut sorted_docs: Vec<(&String, &crate::database::Document)> = base.dir.docs.iter().collect();
  sorted_docs.sort_by_key(|(path, _)| *path);

  for (path, doc) in sorted_docs {
    let document_text = doc.text().to_lowercase();

    // Check if all terms are present in the document
    if terms_lowercase
      .iter()
      .all(|term| document_text.contains(term))
    {
      // Find matching lines
      let mut matching_lines = vec![];
      for (line_num, line) in doc.lines().enumerate() {
        let line_text = line.text.to_lowercase();
        if terms_lowercase.iter().any(|term| line_text.contains(term)) {
          matching_lines.push(format!("  {}: {}", line_num + 1, line.text));
        }
      }

      if !matching_lines.is_empty() {
        // Format: filename\n  line1\n  line2\n
        let mut result_text = path.clone();
        result_text.push('\n');
        result_text.push_str(&matching_lines.join("\n"));
        result_text.push('\n');

        all_results.push(result_text);

        // Also print to stdout for backwards compatibility with CLI usage
        println!("{}", path);
        for line in &matching_lines {
          println!("{}", line);
        }
        println!();
      }
    }
  }

  // Combine all search results into a single fix
  if !all_results.is_empty() {
    let combined_results = all_results.join("\n");
    return Outcome {
      issues: vec![],
      fixes: vec![Fix::SearchResult {
        text: combined_results,
      }],
    };
  }

  Outcome::default()
}
