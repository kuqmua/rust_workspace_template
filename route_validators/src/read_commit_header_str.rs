#[allow(clippy::single_call_fn)] // shared extractor keeps commit-header parsing reusable across commit-check entry points
pub(crate) fn read_commit_header_str(
    headers: crate::domain_types::header_value::AxumHeadersRef<'_>,
) -> Result<crate::domain_types::header_value::HeaderStrRef<'_>, super::CommitError> {
    crate::domain_types::header_value::required_header_str(
        headers,
        super::commit_header_name::COMMIT_HEADER_NAME,
        super::CommitError::no_commit_header,
        |error| {
            super::CommitError::commit_to_str_conversion(
                super::AxumCommitToStrConversionError::from(error),
            )
        },
    )
}
