#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatWithCargofmt {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteDecision {
    False,
    True,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenStreamWriteOutcome {
    Changed,
    Skipped,
    Unchanged,
}
