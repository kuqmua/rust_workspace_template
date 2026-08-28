pub async fn read_bounded_http_response(
    response: super::ReqwestResponse,
    maximum_bytes: super::BoundedReadMaximumBytes,
    concurrency: super::BoundedReadConcurrencyArcSemaphore,
) -> Result<super::BoundedBytes, super::BoundedReadError> {
    let _permit = concurrency
        .0
        .acquire_owned()
        .await
        .map_err(|_error| super::BoundedReadError::LimiterClosed)?;
    let mut inner_response = response.0;
    if let Some(content_length) = inner_response.content_length()
        && content_length > u64::try_from(maximum_bytes.0).unwrap_or(u64::MAX)
    {
        return Err(super::BoundedReadError::ExceedsMaximum { maximum_bytes });
    }
    let initial_capacity = inner_response
        .content_length()
        .and_then(|length| usize::try_from(length).ok())
        .map_or(constants_usize::ZERO, |length| length.min(maximum_bytes.0));
    let mut bytes = Vec::with_capacity(initial_capacity);
    while let Some(chunk) =
        inner_response
            .chunk()
            .await
            .map_err(|source| super::BoundedReadError::Http {
                source: super::ReqwestError::from(source),
            })?
    {
        let next_len = bytes.len().saturating_add(chunk.len());
        super::ensure_size_within_limit(
            super::BoundedReadObservedBytes::from(next_len),
            maximum_bytes,
        )?;
        bytes.extend_from_slice(&chunk);
    }
    Ok(super::BoundedBytes::from(bytes))
}
