#[cfg(test)]
mod tests {
    #[test]
    fn test_timeout_layer_preserves_validated_timeout() {
        let timeout = crate::request_timeout_duration::RequestTimeoutDuration::try_from(
            std::time::Duration::from_secs(1u64),
        )
        .expect("65a8fd30 timeout_layer_preserves_validated_timeout invariant must hold");
        let layer = crate::request_timeout_layer::RequestTimeoutLayer::from(timeout);
        assert_eq!(layer.duration().get(), std::time::Duration::from_secs(1u64));
    }

    #[tokio::test(start_paused = true)]
    async fn test_timeout_response_contains_retry_after_without_text_round_trip() {
        let timeout =
            crate::request_timeout_duration::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(2u64))
                .expect("b140ead4 timeout_response_contains_retry_after_without_text_round_trip invariant must hold");
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
                .expect("9a076c51 timeout_response_contains_retry_after_without_text_round_trip invariant must hold"),
        )
        .await
        .expect("57912096 timeout_response_contains_retry_after_without_text_round_trip invariant must hold");
        assert_eq!(response.status(), http::StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            response.headers().get(http::header::RETRY_AFTER),
            Some(&http::HeaderValue::from_static("2"))
        );
    }
}

// Root-owned module compatibility wrappers.
mod request_timeout_body {}
mod request_timeout_error {}
mod request_timeout_layer {}
mod request_timeout_service {}
mod request_timeout_tower_layer {}
mod std_request_timeout_message {}
