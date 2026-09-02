pub(crate) fn read_commit_header_str(
    headers: crate::axum_headers_ref::AxumHeadersRef<'_>,
) -> Result<crate::header_str_ref::HeaderStrRef<'_>, crate::commit_error::CommitError> {
    crate::required_header_str::required_header_str(
        headers,
        super::commit_header_name::COMMIT_HEADER_NAME,
        || crate::commit_error::CommitError::NoCommitHeader {
            no_commit_header: crate::no_commit_header_message::NoCommitHeaderMessage::from(
                constants_str::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG,
            ),
            location: proc_macro_location_bang::location!(),
        },
        |error| crate::commit_error::CommitError::CommitToStrConversion {
            commit_to_str_conversion:
                crate::axum_commit_to_str_conversion_error::AxumCommitToStrConversionError::from(
                    error,
                ),
            location: proc_macro_location_bang::location!(),
        },
    )
}
