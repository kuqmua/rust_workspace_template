#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq)]
pub enum WritePathOutcome {
    Changed(super::WrittenFilePathBuf),
    Unchanged(super::WrittenFilePathBuf),
}

impl WritePathOutcome {
    #[must_use]
    pub fn into_path(self) -> super::WrittenFilePathBuf {
        match self {
            Self::Changed(path) | Self::Unchanged(path) => path,
        }
    }
    #[must_use]
    pub fn is_changed(&self) -> super::ShouldWriteString {
        super::ShouldWriteString::from(matches!(self, Self::Changed(_)))
    }
    #[must_use]
    pub fn path(&self) -> super::WrittenFilePathRef<'_> {
        match self {
            Self::Changed(path) | Self::Unchanged(path) => {
                super::WrittenFilePathRef::from(path.as_ref())
            }
        }
    }
}
