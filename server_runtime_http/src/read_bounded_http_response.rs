pub async fn read_bounded_http_response(
    response: crate::reqwest_response::ReqwestResponse,
    maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes,
    concurrency: crate::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore,
) -> Result<crate::bounded_bytes::BoundedBytes, crate::bounded_read_error::BoundedReadError> {
    let _permit = concurrency
        .into_inner()
        .acquire_owned()
        .await
        .map_err(|_error| crate::bounded_read_error::BoundedReadError::LimiterClosed)?;
    let mut inner_response = response.into_inner();
    if let Some(content_length) = inner_response.content_length()
        && content_length > u64::try_from(maximum_bytes.get()).unwrap_or(u64::MAX)
    {
        return Err(crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes });
    }
    let initial_capacity = inner_response
        .content_length()
        .and_then(|length| usize::try_from(length).ok())
        .map_or(constants_usize::ZERO, |length| {
            length.min(maximum_bytes.get())
        });
    let mut bytes = Vec::with_capacity(initial_capacity);
    while let Some(chunk) = inner_response.chunk().await.map_err(|source| {
        crate::bounded_read_error::BoundedReadError::Http {
            source: crate::reqwest_error::ReqwestError::from(source),
        }
    })? {
        let next_len = bytes.len().saturating_add(chunk.len());
        crate::ensure_size_within_limit::ensure_size_within_limit(
            crate::bounded_read_observed_bytes::BoundedReadObservedBytes::from(next_len),
            maximum_bytes,
        )?;
        bytes.extend_from_slice(&chunk);
    }
    Ok(crate::bounded_bytes::BoundedBytes::from(bytes))
}
