#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub(crate) struct AssertFilePathRef<'path_lt>(&'path_lt std::path::Path);

impl<'path_lt> From<&'path_lt std::path::PathBuf> for AssertFilePathRef<'path_lt> {
    fn from(value: &'path_lt std::path::PathBuf) -> Self {
        Self(value.as_path())
    }
}
