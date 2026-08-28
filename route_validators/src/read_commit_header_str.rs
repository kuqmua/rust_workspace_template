pub(crate) fn read_commit_header_str(
    headers: crate::header_value::AxumHeadersRef<'_>,
) -> Result<crate::header_value::HeaderStrRef<'_>, super::CommitError> {
    crate::header_value::required_header_str(
        headers,
        super::commit_header_name::COMMIT_HEADER_NAME,
        || super::CommitError::NoCommitHeader {
            no_commit_header: super::NoCommitHeaderMessage::from(
                constants_str::ROUTE_VALIDATORS_NO_COMMIT_HEADER_MSG,
            ),
            location: location_macros::location!(),
        },
        |error| super::CommitError::CommitToStrConversion {
            commit_to_str_conversion: super::AxumCommitToStrConversionError::from(error),
            location: location_macros::location!(),
        },
    )
}
