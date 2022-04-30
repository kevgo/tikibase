pub fn join(path1: &str, path2: &str) -> String {
    if path1.is_empty() || path2.is_empty() {
        format!("{}{}", path1, path2)
    } else {
        format!("{}/{}", path1, path2)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn two_paths() {
        let have = super::join("one", "two");
        let want = "one/two".to_string();
        assert_eq!(have, want);
    }

    #[test]
    fn first_path_empty() {
        let have = super::join("", "two");
        let want = "two".to_string();
        assert_eq!(have, want);
    }

    #[test]
    fn second_path_empty() {
        let have = super::join("one", "");
        let want = "one".to_string();
        assert_eq!(have, want);
    }
}
