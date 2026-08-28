pub(crate) fn cleanup_test_file(path: impl AsRef<std::path::Path>) {
    if let Err(error) = std::fs::remove_file(path.as_ref())
        && error.kind() != std::io::ErrorKind::NotFound
    {
        panic!("33ea4ea2: {error}");
    }
}
