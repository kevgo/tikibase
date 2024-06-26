use super::Result::Fixed;
use crate::check::Location;
use crate::database::Section;
use crate::fix::Fix::SortedSections;
use crate::{fix, Tikibase};

pub fn sort_sections(base: &mut Tikibase, location: Location) -> fix::Result {
  let base_dir = base.root.clone();
  let sections = base.dir.config.sections.clone().unwrap();
  let doc = base.get_doc_mut(&location.file).unwrap();
  doc.content_sections = reorder(&mut doc.content_sections, &sections);
  doc.save(&base_dir);
  Fixed(SortedSections { location })
}

/// drains the given sections vector and provides a new Vector that contains the elements ordered according to schema
fn reorder(sections: &mut Vec<Section>, schema: &[String]) -> Vec<Section> {
  let mut result: Vec<Section> = Vec::new();
  for schema_element in schema {
    let pos = sections
      .iter()
      .position(|section| &section.title_line.text == schema_element);
    match pos {
      None => continue,
      Some(pos) => result.push(sections.remove(pos)),
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::reorder;
  use crate::database::Section;
  use big_s::S;

  #[test]
  fn perfect_match() {
    let schema = vec![S("### one"), S("### two")];
    let mut give: Vec<Section> = vec![
      Section::with_title("### one"),
      Section::with_title("### two"),
    ];
    let have = reorder(&mut give, &schema);
    let have: Vec<&String> = have
      .iter()
      .map(|section| &section.title_line.text)
      .collect();
    assert_eq!(have, vec!["### one", "### two"]);
  }

  #[test]
  fn match_but_missing() {
    let schema = vec![S("### one"), S("### two"), S("### three")];
    let mut give: Vec<Section> = vec![
      Section::with_title("### one"),
      Section::with_title("### three"),
    ];
    let have = reorder(&mut give, &schema);
    let have: Vec<&String> = have
      .iter()
      .map(|section| &section.title_line.text)
      .collect();
    assert_eq!(have, vec!["### one", "### three"]);
  }

  #[test]
  fn wrong_order() {
    let schema = vec![S("### one"), S("### two"), S("### three")];
    let mut give: Vec<Section> = vec![
      Section::with_title("### three"),
      Section::with_title("### two"),
      Section::with_title("### one"),
    ];
    let have = reorder(&mut give, &schema);
    let have: Vec<&String> = have
      .iter()
      .map(|section| &section.title_line.text)
      .collect();
    assert_eq!(have, vec!["### one", "### two", "### three"]);
  }

  #[test]
  fn single_section() {
    let schema = vec![S("### one"), S("### two"), S("### three")];
    let mut give: Vec<Section> = vec![Section::with_title("### three")];
    let have = reorder(&mut give, &schema);
    let have: Vec<&String> = have
      .iter()
      .map(|section| &section.title_line.text)
      .collect();
    assert_eq!(have, vec!["### three"]);
  }
}
