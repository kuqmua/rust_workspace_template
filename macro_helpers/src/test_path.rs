static TEST_SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub(crate) fn test_path(
    test_path_stem: crate::test_path_stem::TestPathStem<'_>,
) -> crate::rs_file_path_buf::RsFilePathBuf {
    let seq = TEST_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    crate::rs_file_path_buf::RsFilePathBuf::from(std::env::temp_dir().join(format!(
        "{}_{}_{}",
        test_path_stem.as_ref(),
        std::process::id(),
        seq
    )))
}
