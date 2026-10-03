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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_async_json_file_read_preserves_parse_size_and_io_error_categories() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(constants_str::CARGO_TOML);
        let invalid_json = crate::read_bounded_json_file_async::read_bounded_json_file_async(
            crate::runtime_path_ref::RuntimePathRef::from(path.as_path()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                constants_usize::VALUE_16_777_216,
            ),
        )
        .await;
        assert!(matches!(
            invalid_json,
            Err(crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(_))
        ));
        let oversized = crate::read_bounded_json_file_async::read_bounded_json_file_async(
            crate::runtime_path_ref::RuntimePathRef::from(path.as_path()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(0usize),
        )
        .await;
        assert!(
            matches!(oversized, Err(crate::bounded_json_read_error::BoundedJsonReadError::Read(
            crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes }
        )) if maximum_bytes.get() == 0usize)
        );
        let invalid_parent = path.join(constants_str::X);
        let io_error = crate::read_bounded_json_file_async::read_bounded_json_file_async(
            crate::runtime_path_ref::RuntimePathRef::from(invalid_parent.as_path()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(0usize),
        )
        .await;
        assert!(matches!(
            io_error,
            Err(crate::bounded_json_read_error::BoundedJsonReadError::Read(
                crate::bounded_read_error::BoundedReadError::Io { .. }
            ))
        ));
    }
}
