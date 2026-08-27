#[cfg(test)]
pub(crate) fn try_write_string_into_path(
    path: impl AsRef<std::path::Path>,
    string_cnt: super::StringFileContentRef<'_>,
) -> std::io::Result<super::WrittenFilePathBuf> {
    super::try_write_string_into_path_with_outcome(path, string_cnt)
        .map(super::WritePathOutcome::into_path)
}
