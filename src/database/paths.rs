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
    let mut uppers: u16 = 0;
    let mut last_slash_pos: usize = 0;
    for (i, current_char) in path.chars().enumerate() {
        if current_char == '/' {
            if last_slash_pos > 0 {
                last_slash_pos += 1;
            };
            match &path[last_slash_pos..i] {
                "." => {}
                ".." => uppers += 1,
                current_segment => {
                    while uppers > 0 {
                        if segments.is_empty() {
                            return Err(());
                        }
                        segments.pop();
                        uppers -= 1;
                    }
                    segments.push(current_segment);
                }
            }
            last_slash_pos = i;
        }
    }
    let start = match last_slash_pos {
        0 => 0,
        other => other + 1,
    };
    match &path[start..] {
        "." => {}
        ".." => uppers += 1,
        last_segment => {
            while uppers > 0 {
                if segments.is_empty() {
                    return Err(());
                }
                segments.pop();
                uppers -= 1;
            }
            segments.push(last_segment);
        }
    }
    while uppers > 0 {
        if segments.is_empty() {
            return Err(());
        }
        segments.pop();
        uppers -= 1;
    }
    Ok(segments.join("/"))
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
