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
            (None, None) => return path1,
            _ => return &path1[..slash_pos],
        }
    }
}

/// provides the directory of the given file path
pub fn dirname(path: &str) -> &str {
    match path.rfind('/') {
        Some(pos) => &path[..pos],
        None => "",
    }
}

/// provides the number of path segments between the given position in the given path and its end
pub fn dirs_between(path: &str, start: usize) -> usize {
    if start == 0 {
        if path.is_empty() {
            return 0;
        }
        return path.matches('/').count() + 1;
    }
    path[start..].matches('/').count()
}

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
                    pop_segments(&mut segments, &mut parents)?;
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
            pop_segments(&mut segments, &mut parents)?;
            segments.push(segment);
        }
    }
    pop_segments(&mut segments, &mut parents)?;
    Ok(segments.join("/"))
}

/// part of normalize
fn pop_segments(segments: &mut Vec<&str>, parents: &mut u16) -> Result<(), ()> {
    while *parents > 0 {
        if segments.is_empty() {
            return Err(());
        }
        segments.pop();
        *parents -= 1;
    }
    Ok(())
}

/// provides the relative path from within given source file to the given target file
pub fn relative(source: &str, target: &str) -> String {
    let common_ancestor = common_anchestor(source, target);
    let source_ups = dirs_between(dirname(source), common_ancestor.len());
    let target_part = path_after(target, common_ancestor.len());
    format!("{}{}", go_up(source_ups), target_part)
}

/// part of `normalize`
fn go_up(count: usize) -> String {
    "../".repeat(count)
}

/// part of `normalize`
fn segment(path: &str, start: usize, end: usize) -> &str {
    if start > 0 {
        &path[start + 1..end]
    } else {
        &path[..end]
    }
}

/// part of `relative`
/// provides the part of the given path after the given prefix
fn path_after(path: &str, pos: usize) -> &str {
    match pos {
        0 => path,
        len if len == path.len() => "",
        len => &path[len + 1..],
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

    mod dirname {

        #[test]
        fn normal() {
            let give = "one/two/file.md";
            let want = "one/two";
            let have = super::super::dirname(give);
            assert_eq!(have, want);
        }

        #[test]
        fn file_only() {
            let give = "file.md";
            let want = "";
            let have = super::super::dirname(give);
            assert_eq!(have, want);
        }

        #[test]
        fn dir_already() {
            let give = "one/two/";
            let want = "one/two";
            let have = super::super::dirname(give);
            assert_eq!(have, want);
        }
    }

    mod dirs_between {

        #[test]
        fn normal() {
            let text = "one/two/three/four/five";
            let have = super::super::dirs_between(text, 7);
            let want = 3;
            assert_eq!(have, want);
        }

        #[test]
        fn full() {
            let text = "one/two/three/four/five";
            let have = super::super::dirs_between(text, 0);
            let want = 5;
            assert_eq!(have, want);
        }

        #[test]
        fn none() {
            let text = "one/two/three/four/five";
            let have = super::super::dirs_between(text, 23);
            let want = 0;
            assert_eq!(have, want);
        }

        #[test]
        fn empty() {
            let have = super::super::dirs_between("", 0);
            let want = 0;
            assert_eq!(have, want);
        }

        #[test]
        fn subdir() {
            let have = super::super::dirs_between("", 0);
            let want = 0;
            assert_eq!(have, want);
        }
    }

    mod go_up {

        #[test]
        fn zero() {
            let have = super::super::go_up(0);
            let want = "".to_string();
            assert_eq!(have, want);
        }

        #[test]
        fn some() {
            let have = super::super::go_up(3);
            let want = "../../../".to_string();
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
            let want = Err(());
            assert_eq!(have, want);
        }
    }

    mod relative {

        #[test]
        fn has_common_ancestors() {
            let path1 = "one/two/three/file.md";
            let path2 = "one/two/tralala/file.md";
            let have = super::super::relative(path1, path2);
            let want = "../tralala/file.md";
            assert_eq!(have, want);
        }

        #[test]
        fn no_common_ancestors() {
            let path1 = "one/two/three/file.md";
            let path2 = "alpha/beta/file.md";
            let have = super::super::relative(path1, path2);
            let want = "../../../alpha/beta/file.md";
            assert_eq!(have, want);
        }

        #[test]
        fn same_dir() {
            let path1 = "one/two/three/file.md";
            let path2 = "one/two/three/other.md";
            let have = super::super::relative(path1, path2);
            let want = "other.md";
            assert_eq!(have, want);
        }

        #[test]
        fn root_dir() {
            let path1 = "file.md";
            let path2 = "other.md";
            let have = super::super::relative(path1, path2);
            let want = "other.md";
            assert_eq!(have, want);
        }

        #[test]
        fn from_subdir() {
            let path1 = "sub/file.md";
            let path2 = "other.md";
            let have = super::super::relative(path1, path2);
            let want = "../other.md";
            assert_eq!(have, want);
        }

        #[test]
        fn into_subdir() {
            let path1 = "file.md";
            let path2 = "sub/other.md";
            let have = super::super::relative(path1, path2);
            let want = "sub/other.md";
            assert_eq!(have, want);
        }
    }

    mod segments_after {

        #[test]
        fn none() {
            let path = "one/two/three/four/five";
            let ancestor = "";
            let have = super::super::path_after(path, ancestor.len());
            let want = "one/two/three/four/five";
            assert_eq!(have, want);
        }

        #[test]
        fn some() {
            let path = "one/two/three/four/five";
            let ancestor = "one/two";
            let have = super::super::path_after(path, ancestor.len());
            let want = "three/four/five";
            assert_eq!(have, want);
        }

        #[test]
        fn full() {
            let path = "one/two/three/four/five";
            let ancestor = "one/two/three/four/five";
            let have = super::super::path_after(path, ancestor.len());
            let want = "";
            assert_eq!(have, want);
        }
    }
}
