#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, PartialEq, Eq)]
pub enum WritePathOutcome {
    Changed(crate::written_file_path_buf::WrittenFilePathBuf),
    Unchanged(crate::written_file_path_buf::WrittenFilePathBuf),
}

impl WritePathOutcome {
    #[must_use]
    pub fn into_path(self) -> crate::written_file_path_buf::WrittenFilePathBuf {
        match self {
            Self::Changed(path) | Self::Unchanged(path) => path,
        }
    }
    #[must_use]
    pub fn is_changed(&self) -> crate::should_write_string::ShouldWriteString {
        crate::should_write_string::ShouldWriteString::from(matches!(self, Self::Changed(_)))
    }
    #[must_use]
    pub fn path(&self) -> crate::written_file_path_ref::WrittenFilePathRef<'_> {
        match self {
            Self::Changed(path) | Self::Unchanged(path) => {
                crate::written_file_path_ref::WrittenFilePathRef::from(path.as_ref())
            }
        }
    }
}
