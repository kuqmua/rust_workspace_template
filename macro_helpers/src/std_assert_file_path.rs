#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefTarget,
    newtype::FromInner,
)]
pub(crate) struct StdAssertFilePath<'path_lt>(&'path_lt std::path::Path);

impl<'path_lt> StdAssertFilePath<'path_lt> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<crate::assert_file_path_ref::AssertFilePathRef<'path_lt>>,
    {
        let path = v.into();
        Self::from(<&'path_lt std::path::Path>::from(path))
    }
}
