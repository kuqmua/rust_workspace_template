pub(super) fn parse_bounded_json_owned(
    bytes: crate::bounded_bytes::BoundedBytes,
) -> Result<
    crate::bounded_json_text::BoundedJsonText,
    crate::bounded_json_read_error::BoundedJsonReadError,
> {
    let text = String::from_utf8(bytes.0).map_err(|error| {
        crate::bounded_json_read_error::BoundedJsonReadError::Read(
            crate::bounded_read_error::BoundedReadError::Utf8 {
                source: crate::bounded_read_from_utf8_error::BoundedReadFromUtf8Error::from(error),
            },
        )
    })?;
    crate::bounded_json_text::BoundedJsonText::try_from(text)
}
