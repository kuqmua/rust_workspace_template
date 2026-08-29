#[cfg(test)]
pub(crate) fn try_write_string_into_path(
    path: impl AsRef<std::path::Path>,
    string_cnt: crate::string_file_content_ref::StringFileContentRef<'_>,
) -> std::io::Result<crate::written_file_path_buf::WrittenFilePathBuf> {
    crate::try_write_string_into_path_with_outcome_tests::try_write_string_into_path_with_outcome(
        path, string_cnt,
    )
    .map(crate::write_path_outcome::WritePathOutcome::into_path)
}
