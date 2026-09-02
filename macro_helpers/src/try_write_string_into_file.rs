pub fn try_write_string_into_file<P>(
    p: P,
    string_file_content_ref: crate::string_file_content_ref::StringFileContentRef<'_>,
) -> std::io::Result<crate::written_file_path_buf::WrittenFilePathBuf>
where
    P: AsRef<std::path::Path>,
{
    crate::try_write_string_into_file_with_outcome::try_write_string_into_file_with_outcome(
        p,
        string_file_content_ref,
    )
    .map(crate::write_path_outcome::WritePathOutcome::into_path)
}
