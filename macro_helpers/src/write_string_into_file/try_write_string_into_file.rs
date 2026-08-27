pub fn try_write_string_into_file<P>(
    file_name: P,
    string_cnt: super::StringFileContentRef<'_>,
) -> std::io::Result<super::WrittenFilePathBuf>
where
    P: AsRef<std::path::Path>,
{
    super::try_write_string_into_file_with_outcome(file_name, string_cnt)
        .map(super::WritePathOutcome::into_path)
}
