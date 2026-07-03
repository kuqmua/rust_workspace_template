#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WritePathOutcome {
    Changed,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RustSourceWritePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringFileContent;
