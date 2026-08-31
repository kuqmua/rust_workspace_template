#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    newtype::AsRefStr,
    newtype::FromInner,
)]
pub(crate) struct ExpectedFileContent<'content_lt>(&'content_lt str);

impl<'content_lt> ExpectedFileContent<'content_lt> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<crate::expected_file_content_ref::ExpectedFileContentRef<'content_lt>>,
    {
        let content = v.into();
        Self::from(<&'content_lt str>::from(content))
    }
}
