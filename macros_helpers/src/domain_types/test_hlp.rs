static TEST_SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct TestPathStemRef<'stem_lt>(&'stem_lt str);
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct TestPathStem<'stem_lt>(&'stem_lt str);
impl<'stem_lt> TestPathStem<'stem_lt> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<TestPathStemRef<'stem_lt>>,
    {
        Self::from(v.into().0)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct AssertFilePathRef<'path_lt>(&'path_lt std::path::Path);
impl<'path_lt> From<&'path_lt std::path::PathBuf> for AssertFilePathRef<'path_lt> {
    fn from(value: &'path_lt std::path::PathBuf) -> Self {
        Self(value.as_path())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct StdAssertFilePath<'path_lt>(&'path_lt std::path::Path);
impl<'path_lt> StdAssertFilePath<'path_lt> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<AssertFilePathRef<'path_lt>>,
    {
        Self::from(v.into().0)
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct ExpectedFileContentRef<'content_lt>(&'content_lt str);
impl<'content_lt> From<&'content_lt String> for ExpectedFileContentRef<'content_lt> {
    fn from(value: &'content_lt String) -> Self {
        Self(value.as_str())
    }
}
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, Clone, Copy, newtype::FromInner)]
pub(crate) struct ExpectedFileContent<'content_lt>(&'content_lt str);
impl<'content_lt> ExpectedFileContent<'content_lt> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<ExpectedFileContentRef<'content_lt>>,
    {
        Self::from(v.into().0)
    }
}
pub(crate) fn test_path(
    stem: TestPathStem<'_>,
) -> crate::domain_types::rs_file_path::RsFilePathBuf {
    let seq = TEST_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    crate::domain_types::rs_file_path::RsFilePathBuf::from(std::env::temp_dir().join(format!(
        "{}_{}_{}",
        stem.0,
        std::process::id(),
        seq
    )))
}
pub(crate) fn cleanup_test_file(path: impl AsRef<std::path::Path>) {
    if let Err(error) = std::fs::remove_file(path.as_ref())
        && error.kind() != std::io::ErrorKind::NotFound
    {
        panic!("33ea4ea2: {error}");
    }
}
pub(crate) fn assert_file_content(path: StdAssertFilePath<'_>, exp: ExpectedFileContent<'_>) {
    let cnt = server_runtime_http::domain_types::read_bounded_file(
        server_runtime_http::domain_types::PathRef::from(path.0),
        server_runtime_http::domain_types::BoundedReadMaximumBytes::from(exp.0.len()),
    )
    .and_then(server_runtime_http::domain_types::BoundedText::try_from)
    .expect("d5ec6712 assert_file_content invariant must hold");
    assert_eq!(cnt.as_ref(), exp.0);
}
