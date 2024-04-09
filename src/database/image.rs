#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Image {
  pub src: String,
  pub line: u32,
  pub start: u32,
  pub end: u32,
}

impl Image {
  pub fn points_to(&self, path: &str) -> bool {
    self.src == path
  }
}

#[cfg(test)]
mod tests {

  mod points_to {
    use crate::database::Image;
    use big_s::S;

    #[test]
    fn matching_image() {
      let img = Image {
        src: S("ok.md"),
        line: 0,
        start: 0,
        end: 0,
      };
      assert!(img.points_to("ok.md"));
    }

    #[test]
    fn mismatching_image() {
      let img = Image {
        src: S("ok.md"),
        line: 0,
        start: 0,
        end: 0,
      };
      assert!(!img.points_to("other.md"));
    }
  }
}
