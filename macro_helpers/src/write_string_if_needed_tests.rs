// The owner module retains lint-sensitive semantics from the original implementation.

#[cfg(test)]
pub(super) fn write_string_if_needed(
    written_file_path_ref: crate::written_file_path_ref::WrittenFilePathRef<'_>,
    string_file_content_ref: crate::string_file_content_ref::StringFileContentRef<'_>,
) -> std::io::Result<crate::should_write_string::ShouldWriteString> {
    let should_write = crate::should_write_string_into_file_tests::should_write_string_into_file(
        written_file_path_ref,
        string_file_content_ref,
    )?;
    if bool::from(should_write) {
        let mut file = atomic_write_file::AtomicWriteFile::open(written_file_path_ref.as_ref())?;
        std::io::Write::write_all(&mut file, string_file_content_ref.as_ref().as_bytes())?;
        file.commit()?;
    }
    Ok(should_write)
}
