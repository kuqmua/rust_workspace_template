#[cfg(test)]
mod tests {
    #[tokio::test(start_paused = true)]
    async fn test_request_timeout_preserves_completed_response_and_inner_error() {
        let completed_matches = match crate::request_timeout_duration::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(1u64)) {
            Ok(timeout) => {
                let inner = tower::service_fn(|_request: axum::extract::Request| {
                    let mut response = axum::response::IntoResponse::into_response(constants_str::X);
                    *response.status_mut() = http::StatusCode::CREATED;
                    let previous = response.headers_mut().insert(http::header::RETRY_AFTER, http::HeaderValue::from_static(constants_str::VALUE_2));
                    assert!(previous.is_none());
                    std::future::ready(Ok::<_, std::convert::Infallible>(response))
                });
                let response_result = tower::ServiceExt::oneshot(
                    crate::request_timeout_service::RequestTimeoutService::new(inner, timeout),
                    http::Request::new(axum::body::Body::empty()),
                ).await;
                let response_matches = match response_result {
                    Ok(response) => {
                        let metadata_matches = response.status() == http::StatusCode::CREATED
                            && response.headers().get(http::header::RETRY_AFTER) == Some(&http::HeaderValue::from_static(constants_str::VALUE_2));
                        metadata_matches && axum::body::to_bytes(response.into_body(), 1usize).await.is_ok_and(|bytes| bytes.as_ref() == constants_str::X.as_bytes())
                    }
                    Err(error) => match error {},
                };
                let failed_inner = tower::service_fn(|_request: axum::extract::Request| std::future::ready(Err::<axum::response::Response, _>(crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero)));
                let error_result = tower::ServiceExt::oneshot(
                    crate::request_timeout_service::RequestTimeoutService::new(failed_inner, timeout),
                    http::Request::new(axum::body::Body::empty()),
                ).await;
                response_matches && matches!(error_result, Err(crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero))
            }
            Err(crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero) => false,
        };
        assert!(completed_matches);
    }

    #[tokio::test(start_paused = true)]
    async fn test_subsecond_request_timeout_drops_inner_future_and_retries_after_one_second() {
        let (sender, mut receiver) = tokio::sync::oneshot::channel::<()>();
        let mut optional_sender = Some(sender);
        let inner = tower::service_fn(move |_request: axum::extract::Request| {
            let retained_sender = optional_sender.take();
            async move {
                let response = std::future::pending::<
                    Result<axum::response::Response, std::convert::Infallible>,
                >()
                .await;
                drop(retained_sender);
                response
            }
        });
        let response_matches =
            match crate::request_timeout_duration::RequestTimeoutDuration::try_from(
                std::time::Duration::from_millis(500u64),
            ) {
                Ok(timeout) => {
                    let service =
                        crate::request_timeout_service::RequestTimeoutService::new(inner, timeout);
                    tower::ServiceExt::oneshot(
                        service,
                        http::Request::new(axum::body::Body::empty()),
                    )
                    .await
                    .is_ok_and(|response| {
                        response.status() == http::StatusCode::SERVICE_UNAVAILABLE
                            && response.headers().get(http::header::RETRY_AFTER)
                                == Some(&http::HeaderValue::from_static(constants_str::VALUE_1))
                    })
                }
                Err(crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero) => {
                    false
                }
            };
        assert!(response_matches);
        assert!(matches!(
            receiver.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Closed)
        ));
    }

    #[test]
    fn test_timeout_layer_preserves_validated_timeout() {
        let timeout = crate::request_timeout_duration::RequestTimeoutDuration::try_from(
            std::time::Duration::from_secs(1u64),
        )
        .expect(constants_str::DIAGNOSTIC_65A8FD30);
        let layer = crate::request_timeout_layer::RequestTimeoutLayer::from(timeout);
        assert_eq!(layer.duration().get(), std::time::Duration::from_secs(1u64));
    }

    #[tokio::test(start_paused = true)]
    async fn test_timeout_response_contains_retry_after_without_text_round_trip() {
        let timeout = crate::request_timeout_duration::RequestTimeoutDuration::try_from(
            std::time::Duration::from_secs(2u64),
        )
        .expect(constants_str::DIAGNOSTIC_B140EAD4);
        let router = axum::Router::from(
            crate::request_timeout_layer::RequestTimeoutLayer::from(timeout).apply(
                crate::axum_router::AxumRouter::from(axum::Router::new().route(
                    constants_str::VALUE_971BB40E,
                    axum::routing::get(async || std::future::pending::<http::StatusCode>().await),
                )),
            ),
        );
        let response = tower::ServiceExt::oneshot(
            router,
            http::Request::builder()
                .uri(constants_str::VALUE_971BB40E)
                .body(axum::body::Body::empty())
                .expect(constants_str::DIAGNOSTIC_9A076C51),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_57912096);
        assert_eq!(response.status(), http::StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            response.headers().get(http::header::RETRY_AFTER),
            Some(&http::HeaderValue::from_static(constants_str::VALUE_2))
        );
    }
}
