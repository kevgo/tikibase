pub fn join(path1: &str, path2: &str) -> String {
    if path1.is_empty() || path2.is_empty() {
        format!("{}{}", path1, path2)
    } else {
        format!("{}/{}", path1, path2)
    }
}

/// removes elements like "../" and "./" from the given string
pub fn normalize(path: &str) -> Result<String, ()> {
    let mut segments: Vec<&str> = vec![];
    let mut parents: u16 = 0;
    let mut segment_start: usize = 0;
    for (i, current_char) in path.chars().enumerate() {
        if current_char == '/' {
            match segment(path, segment_start, i) {
                "." => {}
                ".." => parents += 1,
                segment => {
                    pop_parents(&mut segments, &mut parents)?;
                    segments.push(segment);
                }
            }
            segment_start = i;
        }
    }
    match segment(path, segment_start, path.len()) {
        "." => {}
        ".." => parents += 1,
        segment => {
            pop_parents(&mut segments, &mut parents)?;
            segments.push(segment);
        }
    }
    while parents > 0 {
        if segments.is_empty() {
            return Err(());
        }
        segments.pop();
        parents -= 1;
    }
    Ok(segments.join("/"))
}

fn pop_parents(segments: &mut Vec<&str>, parents: &mut u16) -> Result<(), ()> {
    while *parents > 0 {
        if segments.is_empty() {
            return Err(());
        }
        segments.pop();
        *parents -= 1;
    }
    Ok(())
}

fn segment(path: &str, start: usize, end: usize) -> &str {
    if start > 0 {
        &path[start + 1..end]
    } else {
        &path[..end]
    }
}

#[cfg(test)]
mod tests {

    mod join {
        #[test]
        fn two_paths() {
            let have = super::super::join("one", "two");
            let want = "one/two".to_string();
            assert_eq!(have, want);
        }

        #[test]
        fn first_path_empty() {
            let have = super::super::join("", "two");
            let want = "two".to_string();
            assert_eq!(have, want);
        }

        #[test]
        fn second_path_empty() {
            let have = super::super::join("one", "");
            let want = "one".to_string();
            assert_eq!(have, want);
        }
    }

    mod normalize {

        #[test]
        fn parent_placeholders() {
            let give = "one/three/../two/three/../../new.md";
            let want = Ok("one/new.md".to_string());
            let have = super::super::normalize(give);
            assert_eq!(have, want);
        }

        #[test]
        fn trailing_parent_placeholder() {
            let give = "one/two/three/../..";
            let want = Ok("one".to_string());
            let have = super::super::normalize(give);
            assert_eq!(have, want);
        }

        #[test]
        fn current_placeholders() {
            let give = "./one/./././two/./three.md";
            let want = Ok("one/two/three.md".to_string());
            let have = super::super::normalize(give);
            assert_eq!(have, want);
        }

        #[test]
        fn single_segment() {
            let give = "2.md";
            let want = Ok("2.md".to_string());
            let have = super::super::normalize(give);
            assert_eq!(have, want);
        }

        #[test]
        fn no_placeholders() {
            let give = "one/two/2.md";
            let want = Ok("one/two/2.md".to_string());
            let have = super::super::normalize(give);
            assert_eq!(have, want);
        }

        #[test]
        fn go_above_root() {
            let give = "one/../../1.md";
            let want = Err(());
            let have = super::super::normalize(give);
            assert_eq!(have, want);
        }
    }
}
