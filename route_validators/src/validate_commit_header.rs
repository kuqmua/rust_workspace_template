pub(crate) fn validate_commit_header(
    headers: crate::axum_headers_ref::AxumHeadersRef<'_>,
) -> Result<(), crate::commit_error::CommitError> {
    crate::validate_commit_header_value::validate_commit_header_value(
        crate::read_commit_header_str::read_commit_header_str(headers)?,
    )
}
