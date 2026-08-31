#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct AssertFilePathRef<'path_lt>(&'path_lt std::path::Path);

impl<'path_lt> From<&'path_lt std::path::PathBuf> for AssertFilePathRef<'path_lt> {
    fn from(value: &'path_lt std::path::PathBuf) -> Self {
        Self(value.as_path())
    }
}
