pub async fn read_bounded_file_async(
    path: super::PathRef<'_>,
    maximum_bytes: super::BoundedReadMaximumBytes,
) -> Result<super::BoundedBytes, super::BoundedReadError> {
    let metadata =
        tokio::fs::metadata(path.0)
            .await
            .map_err(|source| super::BoundedReadError::Io {
                source: super::BoundedReadIoError::from(source),
            })?;
    if metadata.len() > u64::try_from(maximum_bytes.0).unwrap_or(u64::MAX) {
        return Err(super::BoundedReadError::ExceedsMaximum { maximum_bytes });
    }
    let bytes = tokio::fs::read(path.0)
        .await
        .map_err(|source| super::BoundedReadError::Io {
            source: super::BoundedReadIoError::from(source),
        })?;
    super::ensure_size_within_limit(
        super::BoundedReadObservedBytes::from(bytes.len()),
        maximum_bytes,
    )?;
    Ok(super::BoundedBytes::from(bytes))
}
