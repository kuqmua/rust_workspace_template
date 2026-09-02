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
        .min(bounded_read_maximum_bytes.get());
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
