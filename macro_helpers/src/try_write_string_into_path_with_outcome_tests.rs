// The owner module retains lint-sensitive semantics from the original implementation.

#[cfg(test)]
pub(crate) fn try_write_string_into_path_with_outcome(
    path: impl AsRef<std::path::Path>,
    string_cnt: crate::string_file_content_ref::StringFileContentRef<'_>,
) -> std::io::Result<crate::write_path_outcome::WritePathOutcome> {
    let path_ref = path.as_ref();
    let should_write = crate::write_string_if_needed_tests::write_string_if_needed(
        crate::written_file_path_ref::WrittenFilePathRef::from(path_ref),
        string_cnt,
    )?;
    let path_buf = crate::written_file_path_buf::WrittenFilePathBuf::from(path_ref.to_path_buf());
    Ok(if bool::from(should_write) {
        crate::write_path_outcome::WritePathOutcome::Changed(path_buf)
    } else {
        crate::write_path_outcome::WritePathOutcome::Unchanged(path_buf)
    })
}
