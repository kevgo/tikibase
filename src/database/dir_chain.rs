use super::Directory;

/// a chain of `Directory` entries, used as the parent directories of a Document or Resource
pub struct DirChain<'a> {
    dirs: Vec<&'a Directory>,
}

// impl DirChain {}
