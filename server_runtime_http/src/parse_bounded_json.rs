pub fn parse_bounded_json(
    bytes: &super::BoundedBytes,
) -> Result<super::BoundedJsonText, super::BoundedJsonReadError> {
    super::parse_bounded_json_owned(bytes.clone())
}
