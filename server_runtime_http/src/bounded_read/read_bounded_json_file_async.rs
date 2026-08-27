pub async fn read_bounded_json_file_async(
    path: super::PathRef<'_>,
    maximum_bytes: super::BoundedReadMaximumBytes,
) -> Result<super::BoundedJsonText, super::BoundedJsonReadError> {
    let bytes = super::read_bounded_file_async(path, maximum_bytes)
        .await
        .map_err(super::BoundedJsonReadError::Read)?;
    super::parse_bounded_json_owned(bytes)
}
