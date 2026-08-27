pub fn try_write_string_into_file_with_outcome<P>(
    file_name: P,
    string_cnt: super::StringFileContentRef<'_>,
) -> std::io::Result<super::WritePathOutcome>
where
    P: AsRef<std::path::Path>,
{
    super::try_write_string_into_path_with_outcome(
        crate::domain_types::rs_file_path::rs_file_path(file_name),
        string_cnt,
    )
}
