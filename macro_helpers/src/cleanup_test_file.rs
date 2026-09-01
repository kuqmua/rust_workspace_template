pub(crate) fn cleanup_test_file(path: impl AsRef<std::path::Path>) {
    if let Err(error) = std::fs::remove_file(path.as_ref())
        && error.kind() != std::io::ErrorKind::NotFound
    {
        std::panic::panic_any(constants_str::PANIC_33EA4EA2.replacen(
            constants_str::PANIC_PLACEHOLDER_81240055,
            error.to_string().as_str(),
            1usize,
        ));
    }
}
