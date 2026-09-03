#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    proc_macro_newtype_as_ref_target::AsRefTarget,
    proc_macro_newtype_from_inner::FromInner,
)]
pub(crate) struct StdAssertFilePath<'path_lt>(&'path_lt std::path::Path);

impl<'path_lt> StdAssertFilePath<'path_lt> {
    pub(crate) fn new<T>(t: T) -> Self
    where
        T: Into<crate::assert_file_path_ref::AssertFilePathRef<'path_lt>>,
    {
        let path = t.into();
        Self::from(<&'path_lt std::path::Path>::from(path))
    }
}
