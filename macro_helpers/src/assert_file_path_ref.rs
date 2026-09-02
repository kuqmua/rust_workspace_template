#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype::AsRefTarget,
    proc_macro_newtype::FromInner,
    proc_macro_newtype::IntoInnerFrom,
)]
pub(crate) struct AssertFilePathRef<'path_lt>(&'path_lt std::path::Path);

impl<'path_lt> From<&'path_lt std::path::PathBuf> for AssertFilePathRef<'path_lt> {
    fn from(path_buf: &'path_lt std::path::PathBuf) -> Self {
        Self(path_buf.as_path())
    }
}
