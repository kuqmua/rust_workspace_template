static TEST_SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub(crate) fn test_path(stem: &str) -> std::path::PathBuf {
    let seq = TEST_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    std::env::temp_dir().join(format!("{stem}_{}_{}", std::process::id(), seq))
}
pub(crate) fn cleanup_test_file(path: impl AsRef<std::path::Path>) {
    if let Err(er) = std::fs::remove_file(path.as_ref())
        && er.kind() != std::io::ErrorKind::NotFound
    {
        panic!("33ea4ea2: {er}");
    }
}
pub(crate) fn assert_file_content(path: &std::path::Path, exp: &str) {
    let cnt = std::fs::read_to_string(path).expect("d5ec6712");
    assert_eq!(cnt, exp);
}
