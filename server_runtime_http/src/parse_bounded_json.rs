pub fn parse_bounded_json(
    bounded_bytes: &crate::bounded_bytes::BoundedBytes,
) -> Result<
    crate::bounded_json_text::BoundedJsonText,
    crate::bounded_json_read_error::BoundedJsonReadError,
> {
    crate::parse_bounded_json_owned::parse_bounded_json_owned(bounded_bytes.clone())
}
