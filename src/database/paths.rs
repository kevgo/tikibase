pub fn join(path1: &str, path2: &str) -> String {
    if path1.is_empty() || path2.is_empty() {
        format!("{}{}", path1, path2)
    } else {
        format!("{}/{}", path1, path2)
    }
}
