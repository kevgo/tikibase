/// joins the given Iterator of Strings into a String with only a single allocation.
pub fn from_iterator<I: Iterator<Item = String>>(iter: I) -> String {
    iter.fold(String::new(), |mut acc, line| {
        acc.push_str(&line);
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::from_iterator;

    #[test]
    fn normal_iterator() {
        let give: Vec<String> = vec!["one".into(), "two".into(), "three".into()];
        let want = "onetwothree".to_string();
        let have = from_iterator(give.into_iter());
        assert_eq!(have, want)
    }

    #[test]
    fn empty_iterator() {
        let give: Vec<String> = vec![];
        let want = "".to_string();
        let have = from_iterator(give.into_iter());
        assert_eq!(have, want)
    }
}
