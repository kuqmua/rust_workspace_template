pub async fn read_bounded_file_async(
    runtime_path_ref: crate::runtime_path_ref::RuntimePathRef<'_>,
    bounded_read_maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes,
) -> Result<crate::bounded_bytes::BoundedBytes, crate::bounded_read_error::BoundedReadError> {
    let file = tokio::fs::File::open(runtime_path_ref.get())
        .await
        .map_err(|source| crate::bounded_read_error::BoundedReadError::Io {
            source: crate::bounded_read_io_error::BoundedReadIoError::from(source),
        })?;
    let metadata = file.metadata().await.map_err(|source| {
        crate::bounded_read_error::BoundedReadError::Io {
            source: crate::bounded_read_io_error::BoundedReadIoError::from(source),
        }
    })?;
    if metadata.len() > u64::try_from(bounded_read_maximum_bytes.get()).unwrap_or(u64::MAX) {
        return Err(
            crate::bounded_read_error::BoundedReadError::ExceedsMaximum {
                maximum_bytes: bounded_read_maximum_bytes,
            },
        );
    }
    let initial_capacity = usize::try_from(metadata.len())
        .unwrap_or_else(|_error| bounded_read_maximum_bytes.get())
        .min(bounded_read_maximum_bytes.get())
        .min(constants_usize::VALUE_4_096);
    let read_limit = u64::try_from(bounded_read_maximum_bytes.get())
        .unwrap_or(u64::MAX)
        .saturating_add(constants_u64::ONE);
    let mut reader = tokio::io::AsyncReadExt::take(file, read_limit);
    let mut bytes = Vec::with_capacity(initial_capacity);
    tokio::io::AsyncReadExt::read_to_end(&mut reader, &mut bytes)
        .await
        .map(|_read_bytes| ())
        .map_err(|source| crate::bounded_read_error::BoundedReadError::Io {
            source: crate::bounded_read_io_error::BoundedReadIoError::from(source),
        })?;
    crate::ensure_size_within_limit::ensure_size_within_limit(
        crate::bounded_read_observed_bytes::BoundedReadObservedBytes::from(bytes.len()),
        bounded_read_maximum_bytes,
    )?;
    Ok(crate::bounded_bytes::BoundedBytes::from(bytes))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_async_bounded_file_read_preserves_directory_and_invalid_parent_errors() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let directory = root.join(constants_str::SRC);
        let invalid_parent = root.join(constants_str::SRC_LIB_RS).join(constants_str::X);
        let preserves_error = async |runtime_path_ref: crate::runtime_path_ref::RuntimePathRef<
            '_,
        >| {
            let expected_kind_option = tokio::fs::read(runtime_path_ref.get())
                .await
                .err()
                .map(|source| source.kind());
            assert!(expected_kind_option.is_some());
            let Some(expected_kind) = expected_kind_option else {
                return false;
            };
            crate::read_bounded_file_async::read_bounded_file_async(runtime_path_ref, crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(constants_usize::VALUE_16_777_216)).await.is_err_and(|error| matches!(error, crate::bounded_read_error::BoundedReadError::Io { source } if source.kind() == expected_kind))
        };
        assert!(
            preserves_error(crate::runtime_path_ref::RuntimePathRef::from(
                directory.as_path()
            ))
            .await
        );
        assert!(
            preserves_error(crate::runtime_path_ref::RuntimePathRef::from(
                invalid_parent.as_path()
            ))
            .await
        );
    }
    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_async_bounded_read_rejects_content_beyond_reported_zero_length() {
        let path = std::path::Path::new(constants_str::TEST_PROCESS_COMMAND_LINE_PATH);
        let metadata_result = tokio::fs::metadata(path).await;
        assert!(metadata_result.is_ok_and(|metadata| metadata.len() == 0u64));
        let maximum = crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(0usize);
        let result = crate::read_bounded_file_async::read_bounded_file_async(
            crate::runtime_path_ref::RuntimePathRef::from(path),
            maximum,
        )
        .await;
        assert!(
            matches!(result, Err(crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes }) if maximum_bytes == maximum)
        );
    }
}
