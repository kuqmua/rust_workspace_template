pub(super) fn parse_bounded_json_owned(
    bytes: super::BoundedBytes,
) -> Result<super::BoundedJsonText, super::BoundedJsonReadError> {
    let text = String::from_utf8(bytes.0).map_err(|error| {
        super::BoundedJsonReadError::Read(super::BoundedReadError::Utf8 {
            source: super::BoundedReadFromUtf8Error::from(error),
        })
    })?;
    super::BoundedJsonText::try_from(text)
}
