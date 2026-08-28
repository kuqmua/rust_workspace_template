// The owner module retains lint-sensitive semantics from the original implementation.

#[cfg(test)]
pub(super) fn write_string_if_needed(
    path: super::WrittenFilePathRef<'_>,
    string_cnt: super::StringFileContentRef<'_>,
) -> std::io::Result<super::ShouldWriteString> {
    let should_write = super::should_write_string_into_file(path, string_cnt)?;
    if bool::from(should_write) {
        let mut file = atomic_write_file::AtomicWriteFile::open(path.as_ref())?;
        std::io::Write::write_all(&mut file, string_cnt.as_ref().as_bytes())?;
        file.commit()?;
    }
    Ok(should_write)
}
