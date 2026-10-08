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
            length
                .min(bounded_read_maximum_bytes.get())
                .min(constants_usize::VALUE_4_096)
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

#[cfg(test)]
mod tests {
    fn response_read_semaphore_fixture()
    -> crate::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore {
        crate::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore::new(
            crate::bounded_read_concurrency_maximum_non_zero_usize::BoundedReadConcurrencyMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        )
    }

    #[tokio::test]
    async fn test_response_reads_release_permit_after_success_and_size_errors() {
        let bounded_read_concurrency_arc_semaphore = response_read_semaphore_fixture();
        let read = async |bounded_read_maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes,
                          bounded_read_observed_bytes: Option<crate::bounded_read_observed_bytes::BoundedReadObservedBytes>| {
            let mut response = http::Response::new(constants_str::ABCD_ALT);
            if let Some(content_length) = bounded_read_observed_bytes {
                assert!(http::HeaderValue::try_from(content_length.get().to_string()).is_ok_and(|header_value| response.headers_mut().insert(http::header::CONTENT_LENGTH, header_value).is_none()));
            }
            let result = crate::read_bounded_http_response::read_bounded_http_response(
                crate::reqwest_response::ReqwestResponse::from(reqwest::Response::from(response)),
                bounded_read_maximum_bytes,
                bounded_read_concurrency_arc_semaphore.clone(),
            ).await;
            let expected_result = if bounded_read_maximum_bytes.get() == 4usize {
                result.is_ok_and(|bounded_bytes| bounded_bytes.into_inner() == constants_str::ABCD_ALT.as_bytes())
            } else {
                matches!(result, Err(crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes }) if maximum_bytes == bounded_read_maximum_bytes)
            };
            expected_result && bounded_read_concurrency_arc_semaphore.clone().into_inner().available_permits() == 1usize
        };
        assert!(
            read(
                crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(4usize),
                None
            )
            .await
        );
        assert!(
            read(
                crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(3usize),
                None
            )
            .await
        );
        assert!(
            read(
                crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(3usize),
                Some(crate::bounded_read_observed_bytes::BoundedReadObservedBytes::from(4usize))
            )
            .await
        );
    }

    #[tokio::test]
    async fn test_closed_response_read_limiter_precedes_payload_size_validation() {
        let limiter = response_read_semaphore_fixture();
        limiter.clone().into_inner().close();
        let result = crate::read_bounded_http_response::read_bounded_http_response(
            crate::reqwest_response::ReqwestResponse::from(reqwest::Response::from(
                http::Response::new(constants_str::X),
            )),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(0usize),
            limiter,
        )
        .await;
        assert!(matches!(
            result,
            Err(crate::bounded_read_error::BoundedReadError::LimiterClosed)
        ));
    }

    #[tokio::test]
    async fn test_cancelled_response_read_waiter_does_not_consume_released_permit() {
        let limiter = response_read_semaphore_fixture();
        let permit = match limiter.clone().into_inner().acquire_owned().await {
            Ok(permit) => permit,
            Err(error) => {
                assert_eq!(error.to_string(), constants_str::EMPTY);
                return;
            }
        };
        let mut waiting_read = Box::pin(
            crate::read_bounded_http_response::read_bounded_http_response(
                crate::reqwest_response::ReqwestResponse::from(reqwest::Response::from(
                    http::Response::new(constants_str::X),
                )),
                crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(1usize),
                limiter.clone(),
            ),
        );
        let pending = std::future::poll_fn(|context| {
            std::task::Poll::Ready(waiting_read.as_mut().poll(context).is_pending())
        })
        .await;
        assert!(pending);
        drop(waiting_read);
        assert_eq!(limiter.clone().into_inner().available_permits(), 0usize);
        drop(permit);
        assert_eq!(limiter.clone().into_inner().available_permits(), 1usize);
        let result = crate::read_bounded_http_response::read_bounded_http_response(
            crate::reqwest_response::ReqwestResponse::from(reqwest::Response::from(
                http::Response::new(constants_str::X),
            )),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(1usize),
            limiter.clone(),
        )
        .await;
        assert!(result.is_ok_and(|bytes| bytes.into_inner() == constants_str::X.as_bytes()));
        assert_eq!(limiter.into_inner().available_permits(), 1usize);
    }
    #[tokio::test]
    async fn test_response_zero_limit_accepts_empty_body_and_releases_permits_after_rejection() {
        let limiter = response_read_semaphore_fixture();
        let maximum = crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(0usize);
        let empty = crate::read_bounded_http_response::read_bounded_http_response(
            crate::reqwest_response::ReqwestResponse::from(reqwest::Response::from(
                http::Response::new(constants_str::EMPTY),
            )),
            maximum,
            limiter.clone(),
        )
        .await;
        assert!(empty.is_ok_and(|bytes| bytes.into_inner().is_empty()));
        assert_eq!(limiter.clone().into_inner().available_permits(), 1usize);
        let nonempty = crate::read_bounded_http_response::read_bounded_http_response(
            crate::reqwest_response::ReqwestResponse::from(reqwest::Response::from(
                http::Response::new(constants_str::X),
            )),
            maximum,
            limiter.clone(),
        )
        .await;
        assert!(
            matches!(nonempty, Err(crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes }) if maximum_bytes == maximum)
        );
        assert_eq!(limiter.into_inner().available_permits(), 1usize);
    }
}
