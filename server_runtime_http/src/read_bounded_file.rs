pub fn read_bounded_file(
    path: super::PathRef<'_>,
    maximum_bytes: super::BoundedReadMaximumBytes,
) -> Result<super::BoundedBytes, super::BoundedReadError> {
    let file = std::fs::File::open(path.0).map_err(|source| super::BoundedReadError::Io {
        source: super::BoundedReadIoError::from(source),
    })?;
    let metadata = file
        .metadata()
        .map_err(|source| super::BoundedReadError::Io {
            source: super::BoundedReadIoError::from(source),
        })?;
    if metadata.len() > u64::try_from(maximum_bytes.0).unwrap_or(u64::MAX) {
        return Err(super::BoundedReadError::ExceedsMaximum { maximum_bytes });
    }
    let initial_capacity = usize::try_from(metadata.len())
        .unwrap_or(maximum_bytes.0)
        .min(maximum_bytes.0);
    let read_limit = u64::try_from(maximum_bytes.0)
        .unwrap_or(u64::MAX)
        .saturating_add(constants_u64::ONE);
    let mut reader = std::io::Read::take(file, read_limit);
    let mut bytes = Vec::with_capacity(initial_capacity);
    std::io::Read::read_to_end(&mut reader, &mut bytes)
        .map(|_read_bytes| ())
        .map_err(|source| super::BoundedReadError::Io {
            source: super::BoundedReadIoError::from(source),
        })?;
    super::ensure_size_within_limit(
        super::BoundedReadObservedBytes::from(bytes.len()),
        maximum_bytes,
    )?;
    Ok(super::BoundedBytes::from(bytes))
}
