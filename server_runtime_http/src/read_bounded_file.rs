pub fn read_bounded_file(
    runtime_path_ref: crate::runtime_path_ref::RuntimePathRef<'_>,
    bounded_read_maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes,
) -> Result<crate::bounded_bytes::BoundedBytes, crate::bounded_read_error::BoundedReadError> {
    let file = std::fs::File::open(runtime_path_ref.get()).map_err(|source| {
        crate::bounded_read_error::BoundedReadError::Io {
            source: crate::bounded_read_io_error::BoundedReadIoError::from(source),
        }
    })?;
    let metadata =
        file.metadata()
            .map_err(|source| crate::bounded_read_error::BoundedReadError::Io {
                source: crate::bounded_read_io_error::BoundedReadIoError::from(source),
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
    let mut reader = std::io::Read::take(file, read_limit);
    let mut bytes = Vec::with_capacity(initial_capacity);
    std::io::Read::read_to_end(&mut reader, &mut bytes)
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
    #[cfg(target_os = "linux")]
    #[test]
    fn test_sync_bounded_reader_accepts_empty_input_with_zero_limit() {
        assert!(
            crate::read_bounded_file::read_bounded_file(
                crate::runtime_path_ref::RuntimePathRef::from(std::path::Path::new(
                    constants_str::TEST_EMPTY_DEVICE_PATH,
                )),
                crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(0usize),
            )
            .is_ok_and(|bounded_bytes| bounded_bytes.into_inner().is_empty())
        );
    }

    #[test]
    fn test_bounded_file_read_preserves_directory_and_invalid_parent_io_errors() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let directory = root.join(constants_str::SRC);
        let invalid_parent = root.join(constants_str::SRC_LIB_RS).join(constants_str::X);
        assert!([directory, invalid_parent].into_iter().all(|path| {
            std::fs::read(path.as_path()).is_err_and(|expected| {
                crate::read_bounded_file::read_bounded_file(
                    crate::runtime_path_ref::RuntimePathRef::from(path.as_path()),
                    crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                        constants_usize::VALUE_16_777_216,
                    ),
                )
                .is_err_and(|actual| {
                    matches!(actual,
                        crate::bounded_read_error::BoundedReadError::Io { source }
                            if source.kind() == expected.kind()
                    )
                })
            })
        }));
    }
    #[cfg(target_os = "linux")]
    #[test]
    fn test_sync_bounded_read_rejects_content_beyond_reported_zero_length() {
        let path = std::path::Path::new(constants_str::TEST_PROCESS_COMMAND_LINE_PATH);
        let metadata_result = std::fs::metadata(path);
        assert!(metadata_result.is_ok_and(|metadata| metadata.len() == 0u64));
        let maximum = crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(0usize);
        let result = crate::read_bounded_file::read_bounded_file(
            crate::runtime_path_ref::RuntimePathRef::from(path),
            maximum,
        );
        assert!(
            matches!(result, Err(crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes }) if maximum_bytes == maximum)
        );
    }
}
