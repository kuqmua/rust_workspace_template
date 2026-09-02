pub async fn read_bounded_json_file_async(
    runtime_path_ref: crate::runtime_path_ref::RuntimePathRef<'_>,
    bounded_read_maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes,
) -> Result<
    crate::bounded_json_text::BoundedJsonText,
    crate::bounded_json_read_error::BoundedJsonReadError,
> {
    let bytes = crate::read_bounded_file_async::read_bounded_file_async(
        runtime_path_ref,
        bounded_read_maximum_bytes,
    )
    .await
    .map_err(crate::bounded_json_read_error::BoundedJsonReadError::Read)?;
    crate::parse_bounded_json_owned::parse_bounded_json_owned(bytes)
}
