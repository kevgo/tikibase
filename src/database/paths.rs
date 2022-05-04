/// provides the largest common ancestor for the two given paths
fn common_anchestor<'a, 'b>(path1: &'a str, path2: &'b str) -> &'a str {
    let mut path1_chars = path1.char_indices();
    let mut path2_chars = path2.chars();
    let mut slash_pos: usize = 0;
    loop {
        match (path1_chars.next(), path2_chars.next()) {
            (Some((pos, seg1)), Some(seg2)) if seg1 == seg2 => {
                if seg1 == '/' {
                    slash_pos = pos;
                }
            }
            (None, None) => return path1.into(),
            _ => return &path1[..slash_pos],
        }
    }
}

/// provides the directory of the given file path
// pub fn dirname(path: &str) -> &str {
//     match path.rfind('/') {
//         Some(pos) => &path[..pos],
//         None => path,
//     }
// }

pub fn join(path1: &str, path2: &str) -> String {
    if path1.is_empty() || path2.is_empty() {
        format!("{}{}", path1, path2)
    } else {
        format!("{}/{}", path1, path2)
    }
}

/// resolves elements like "../" and "./" in the given string
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

/// part of normalize
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

pub fn relative(source: &str, target: &str) -> String {
    let ancestor = common_anchestor(source, target);
    let pos = ancestor.len();

    // example: source = "one/two/three/four/five.md"
    //          target = "one/two/alpha/beta.md"
    // determine highest common directory
    //   - iterate path segments from the root upwards while both source and target have the same segments
    //   - example: highest common parent is "one/two/""
    // determine source and target directories
    //   - cut off the last path segment
    //   - example: source dir is "one/two/three/four/"
    //              parent dir is "one/two/alpha"
    // go down from the source directory to the highest common parent directory
    //   - for each directory between the source directory and the highest common parent, add ".." to the result
    //   - example: we have to go two directories up to get from the source dir to parent --> result = "../../"
    // add the path segments from the highest common parent to the target directory
    //   - example: to get from parent ("one/two/") to target dir, we have to add "alpha" to result
    // the relative path from "one/two/three/four/" to "one/two/alpha" is "../../alpha"
    "".into()
}

/// part of `normalize`
fn segment(path: &str, start: usize, end: usize) -> &str {
    if start > 0 {
        &path[start + 1..end]
    } else {
        &path[..end]
    }
}

#[cfg(test)]
mod tests {

    mod common_anchestor {

        #[test]
        fn has_common_ancestors() {
            let path1 = "one/two/three/file.md";
            let path2 = "one/two/throw/file.md";
            let have = super::super::common_anchestor(path1, path2);
            let want = "one/two";
            assert_eq!(have, want);
        }

        #[test]
        fn no_common_ancestors() {
            let path1 = "one/two/three/file.md";
            let path2 = "alpha/beta/file.md";
            let have = super::super::common_anchestor(path1, path2);
            let want = "";
            assert_eq!(have, want);
        }

        #[test]
        fn identical() {
            let path1 = "one/two/three/file.md";
            let path2 = "one/two/three/file.md";
            let have = super::super::common_anchestor(path1, path2);
            let want = "one/two/three/file.md";
            assert_eq!(have, want);
        }
    }

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
            let have = super::super::normalize(give);
            assert_eq!(have, want);
        }
    }

    mod relative {

        #[test]
        fn has_common_ancestors() {
            let path1 = "one/two/three/file.md";
            let path2 = "one/two/throw/file.md";
            let have = super::super::relative(path1, path2);
            let want = "../throw/file.md";
            assert_eq!(have, want);
        }

        #[test]
        fn no_common_ancestors() {
            let path1 = "one/two/three/file.md";
            let path2 = "alpha/beta/file.md";
            let have = super::super::common_anchestor(path1, path2);
            let want = "../../../alpha/beta/file.md";
            assert_eq!(have, want);
        }

        #[test]
        fn same_dir() {
            let path1 = "one/two/three/file.md";
            let path2 = "one/two/three/other.md";
            let have = super::super::common_anchestor(path1, path2);
            let want = "other.md";
            assert_eq!(have, want);
        }
    }
}
