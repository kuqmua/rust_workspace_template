pub async fn read_bounded_json_http_response(
    reqwest_response: crate::reqwest_response::ReqwestResponse,
    bounded_read_maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes,
    bounded_read_concurrency_arc_semaphore: crate::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore,
) -> Result<
    crate::bounded_json_text::BoundedJsonText,
    crate::bounded_json_read_error::BoundedJsonReadError,
> {
    let bytes = crate::read_bounded_http_response::read_bounded_http_response(
        reqwest_response,
        bounded_read_maximum_bytes,
        bounded_read_concurrency_arc_semaphore,
    )
    .await
    .map_err(crate::bounded_json_read_error::BoundedJsonReadError::Read)?;
    crate::parse_bounded_json_owned::parse_bounded_json_owned(bytes)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_closed_json_response_limiter_precedes_body_validation() {
        let semaphore = crate::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore::new(
            crate::bounded_read_concurrency_maximum_non_zero_usize::BoundedReadConcurrencyMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
        ).into_inner();
        semaphore.close();
        let response = http::Response::new(vec![255u8]);
        let result = crate::read_bounded_json_http_response::read_bounded_json_http_response(
            crate::reqwest_response::ReqwestResponse::from(reqwest::Response::from(response)),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(0usize),
            crate::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore::from(
                semaphore,
            ),
        )
        .await;
        assert!(matches!(
            result,
            Err(crate::bounded_json_read_error::BoundedJsonReadError::Read(
                crate::bounded_read_error::BoundedReadError::LimiterClosed
            ))
        ));
    }

    #[tokio::test]
    async fn test_json_http_response_preserves_text_and_error_categories() {
        let read = async |bounded_bytes: crate::bounded_bytes::BoundedBytes,
                          bounded_read_maximum_bytes: crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes| {
            let response = http::Response::new(bounded_bytes.into_inner());
            crate::read_bounded_json_http_response::read_bounded_json_http_response(
                crate::reqwest_response::ReqwestResponse::from(reqwest::Response::from(response)),
                bounded_read_maximum_bytes,
                crate::bounded_read_concurrency_arc_semaphore::BoundedReadConcurrencyArcSemaphore::new(
                    crate::bounded_read_concurrency_maximum_non_zero_usize::BoundedReadConcurrencyMaximumNonZeroUsize::from(std::num::NonZeroUsize::MIN),
                ),
            ).await
        };
        let text = constants_str::TEST_JSON_MAP_WITH_ONE_ENTRY;
        let valid = read(
            crate::bounded_bytes::BoundedBytes::from(text.as_bytes().to_vec()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(text.len()),
        )
        .await;
        assert!(valid.is_ok_and(|bounded_json_text| bounded_json_text.as_ref() == text));
        let invalid_json = read(
            crate::bounded_bytes::BoundedBytes::from(constants_str::X.as_bytes().to_vec()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(1usize),
        )
        .await;
        assert!(matches!(
            invalid_json,
            Err(crate::bounded_json_read_error::BoundedJsonReadError::SerdeJson(_))
        ));
        let invalid_utf8 = read(
            crate::bounded_bytes::BoundedBytes::from(vec![255u8]),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(1usize),
        )
        .await;
        assert!(matches!(
            invalid_utf8,
            Err(crate::bounded_json_read_error::BoundedJsonReadError::Read(
                crate::bounded_read_error::BoundedReadError::Utf8 { .. }
            ))
        ));
        let oversized = read(
            crate::bounded_bytes::BoundedBytes::from(text.as_bytes().to_vec()),
            crate::bounded_read_maximum_bytes::BoundedReadMaximumBytes::from(
                text.len().saturating_sub(1usize),
            ),
        )
        .await;
        assert!(
            matches!(oversized, Err(crate::bounded_json_read_error::BoundedJsonReadError::Read(crate::bounded_read_error::BoundedReadError::ExceedsMaximum { maximum_bytes })) if maximum_bytes.get() == text.len().saturating_sub(1usize))
        );
    }
}
