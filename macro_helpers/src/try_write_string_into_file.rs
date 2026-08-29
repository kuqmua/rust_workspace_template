pub fn try_write_string_into_file<P>(
    file_name: P,
    string_cnt: crate::string_file_content_ref::StringFileContentRef<'_>,
) -> std::io::Result<crate::written_file_path_buf::WrittenFilePathBuf>
where
    P: AsRef<std::path::Path>,
{
    crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(
        file_name, string_cnt,
    )
    .map(crate::write_path_outcome::WritePathOutcome::into_path)
}
