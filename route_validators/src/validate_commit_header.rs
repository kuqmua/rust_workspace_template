pub(crate) fn validate_commit_header(
    headers: crate::header_value::AxumHeadersRef<'_>,
) -> Result<(), super::CommitError> {
    super::validate_commit_header_value(super::read_commit_header_str(headers)?)
}
