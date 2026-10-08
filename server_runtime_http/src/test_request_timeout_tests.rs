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
    async fn test_timeout_response_preserves_json_contract_and_truncates_fractional_retry_delay() {
        let timeout = crate::request_timeout_duration::RequestTimeoutDuration::try_from(
            std::time::Duration::new(2u64, 999_999_999u32),
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
        assert_eq!(
            response.headers().get(http::header::CONTENT_TYPE),
            Some(&http::HeaderValue::from_static(
                constants_str::APPLICATION_JSON
            ))
        );
        let expected = serde_json::json!({
            (constants_str::ERROR.to_ascii_lowercase()): constants_str::REQUEST_TIMEOUT
        });
        assert!(
            axum::body::to_bytes(response.into_body(), expected.to_string().len())
                .await
                .is_ok_and(|bytes| {
                    serde_json::from_slice::<serde_json::Value>(&bytes)
                        .is_ok_and(|json| json == expected)
                })
        );
    }

    #[test]
    fn test_service_request_timeout_preserves_positive_duration_boundaries() {
        assert!(
            [
                std::time::Duration::from_nanos(1u64),
                std::time::Duration::new(1u64, 123_456_789u32),
                std::time::Duration::new(u64::MAX, 0u32),
                std::time::Duration::MAX,
            ]
            .into_iter()
            .all(|duration| {
                crate::request_timeout_duration::RequestTimeoutDuration::try_from(duration)
                    .is_ok_and(|timeout| timeout.get() == duration)
            })
        );
    }

    #[test]
    fn test_http_middleware_services_forward_ready_pending_and_error_readiness() {
        #[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
        enum TestHttpMiddlewareReadiness {
            Failed,
            Pending,
            Ready,
        }
        #[derive(
            proc_macro_optimal_memory_layout::OptimalMemoryLayout,
            proc_macro_newtype_from_inner::FromInner,
        )]
        struct TestHttpMiddlewareReadinessService(TestHttpMiddlewareReadiness);
        impl tower::Service<axum::extract::Request> for TestHttpMiddlewareReadinessService {
            type Error = crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError;
            type Future = std::future::Ready<Result<Self::Response, Self::Error>>;
            type Response = axum::response::Response;
            fn call(&mut self, _request: axum::extract::Request) -> Self::Future {
                std::future::ready(Err(crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero))
            }
            fn poll_ready(
                &mut self,
                _context: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Result<(), Self::Error>> {
                match self.0 {
                    TestHttpMiddlewareReadiness::Failed => std::task::Poll::Ready(Err(crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero)),
                    TestHttpMiddlewareReadiness::Pending => std::task::Poll::Pending,
                    TestHttpMiddlewareReadiness::Ready => std::task::Poll::Ready(Ok(())),
                }
            }
        }
        assert!(crate::request_timeout_duration::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(1u64)).is_ok_and(|timeout| {
            [
                (TestHttpMiddlewareReadiness::Failed, std::task::Poll::Ready(Err(crate::std_request_timeout_try_from_duration_error::StdRequestTimeoutTryFromDurationError::Zero))),
                (TestHttpMiddlewareReadiness::Pending, std::task::Poll::Pending),
                (TestHttpMiddlewareReadiness::Ready, std::task::Poll::Ready(Ok(()))),
            ].into_iter().all(|(readiness, expected)| {
                let mut service = crate::request_timeout_service::RequestTimeoutService::new(TestHttpMiddlewareReadinessService::from(readiness), timeout);
                let mut context = std::task::Context::from_waker(std::task::Waker::noop());
                let mut request_id_service = crate::request_id_service::RequestIdService::new(TestHttpMiddlewareReadinessService::from(readiness), None);
                let paths = crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc::from(crate::http_metrics_path_cache::HttpMetricsPathCache::from(crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum::from(std::num::NonZeroUsize::MIN)));
                let mut metrics_service = crate::http_metrics_service::HttpMetricsService::new(TestHttpMiddlewareReadinessService::from(readiness), paths);
                let mut security_service = crate::security_headers_service::SecurityHeadersService::new(None, crate::forwarded_proto_trust::ForwardedProtoTrust::Trust, TestHttpMiddlewareReadinessService::from(readiness));
                tower::Service::poll_ready(&mut service, &mut context) == expected
                    && tower::Service::poll_ready(&mut request_id_service, &mut context) == expected
                    && tower::Service::poll_ready(&mut metrics_service, &mut context) == expected
                    && tower::Service::poll_ready(&mut security_service, &mut context) == expected
            })
        }));
    }
}
