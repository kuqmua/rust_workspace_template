pub async fn read_bounded_http_response(
    reqwest_response: crate::reqwest_response::ReqwestResponse,
    bounded_read_maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes,
    bounded_read_concurrency_arc_semaphore: crate::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore,
) -> Result<crate::bounded_bytes::BoundedBytes, crate::bounded_read_error::BoundedReadError> {
    let _permit = bounded_read_concurrency_arc_semaphore
        .into_inner()
        .acquire_owned()
        .await
        .map_err(|_error| crate::bounded_read_error::BoundedReadError::LimiterClosed)?;
    let mut inner_response = reqwest_response.into_inner();
    if let Some(content_length) = inner_response.content_length()
        && content_length > u64::try_from(bounded_read_maximum_bytes.get()).unwrap_or(u64::MAX)
    {
        return Err(
            crate::bounded_read_error::BoundedReadError::ExceedsMaximum {
                maximum_bytes: bounded_read_maximum_bytes,
            },
        );
    }
    let initial_capacity = inner_response
        .content_length()
        .and_then(|length| usize::try_from(length).ok())
        .map_or(constants_usize::ZERO, |length| {
            length.min(bounded_read_maximum_bytes.get())
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
            bounded_read_maximum_bytes,
        )?;
        bytes.extend_from_slice(&chunk);
    }
    Ok(crate::bounded_bytes::BoundedBytes::from(bytes))
}
