static TEST_SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[derive(Debug, Clone, Copy)]
pub(crate) struct TestPathStemRef<'stem_lt>(&'stem_lt str);
impl<'stem_lt> From<&'stem_lt str> for TestPathStemRef<'stem_lt> {
    fn from(value: &'stem_lt str) -> Self {
        Self(value)
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct TestPathStem<'stem_lt>(&'stem_lt str);
impl<'stem_lt> TestPathStem<'stem_lt> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<TestPathStemRef<'stem_lt>>,
    {
        Self(v.into().0)
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct StdAssertFilePathRef<'path_lt>(&'path_lt std::path::Path);
impl<'path_lt> From<&'path_lt std::path::Path> for StdAssertFilePathRef<'path_lt> {
    fn from(value: &'path_lt std::path::Path) -> Self {
        Self(value)
    }
}
impl<'path_lt> From<&'path_lt std::path::PathBuf> for StdAssertFilePathRef<'path_lt> {
    fn from(value: &'path_lt std::path::PathBuf) -> Self {
        Self(value.as_path())
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct StdAssertFilePath<'path_lt>(&'path_lt std::path::Path);
impl<'path_lt> StdAssertFilePath<'path_lt> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<StdAssertFilePathRef<'path_lt>>,
    {
        Self(v.into().0)
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExpectedFileContentRef<'content_lt>(&'content_lt str);
impl<'content_lt> From<&'content_lt str> for ExpectedFileContentRef<'content_lt> {
    fn from(value: &'content_lt str) -> Self {
        Self(value)
    }
}
impl<'content_lt> From<&'content_lt String> for ExpectedFileContentRef<'content_lt> {
    fn from(value: &'content_lt String) -> Self {
        Self(value.as_str())
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct ExpectedFileContent<'content_lt>(&'content_lt str);
impl<'content_lt> ExpectedFileContent<'content_lt> {
    pub(crate) fn new<T>(v: T) -> Self
    where
        T: Into<ExpectedFileContentRef<'content_lt>>,
    {
        Self(v.into().0)
    }
}
pub(crate) fn test_path(stem: TestPathStem<'_>) -> crate::rs_file_path::StdRsFilePath {
    let seq = TEST_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    crate::rs_file_path::StdRsFilePath(std::env::temp_dir().join(format!(
        "{}_{}_{}",
        stem.0,
        std::process::id(),
        seq
    )))
}
pub(crate) fn cleanup_test_file(path: impl AsRef<std::path::Path>) {
    if let Err(er) = std::fs::remove_file(path.as_ref())
        && er.kind() != std::io::ErrorKind::NotFound
    {
        panic!("33ea4ea2: {er}");
    }
}
pub(crate) fn assert_file_content(path: StdAssertFilePath<'_>, exp: ExpectedFileContent<'_>) {
    let cnt = std::fs::read_to_string(path.0).expect("d5ec6712");
    assert_eq!(cnt, exp.0);
}
