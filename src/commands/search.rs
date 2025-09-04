use super::Outcome;
use crate::Tikibase;

#[must_use]
pub fn search(base: &Tikibase, terms: &[String]) -> Outcome {
  if terms.is_empty() {
    println!("No search terms provided");
    return Outcome::default();
  }

  let terms_lowercase: Vec<String> = terms.iter().map(|t| t.to_lowercase()).collect();

  for (path, doc) in &base.dir.docs {
    let document_text = doc.text().to_lowercase();

    // Check if all terms are present in the document
    if terms_lowercase
      .iter()
      .all(|term| document_text.contains(term))
    {
      println!("{}", path);

      // Print matching lines
      for (line_num, line) in doc.lines().enumerate() {
        let line_text = line.text.to_lowercase();
        if terms_lowercase.iter().any(|term| line_text.contains(term)) {
          println!("  {}: {}", line_num + 1, line.text);
        }
      }
      println!();
    }
  }

  Outcome::default()
}
