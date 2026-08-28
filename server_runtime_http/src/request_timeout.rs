#[path = "request_timeout_body.rs"]
mod request_timeout_body;
#[path = "request_timeout_error.rs"]
mod request_timeout_error;
#[path = "request_timeout_layer.rs"]
mod request_timeout_layer;
#[path = "request_timeout_service.rs"]
mod request_timeout_service;
#[path = "request_timeout_tower_layer.rs"]
mod request_timeout_tower_layer;
#[path = "std_request_timeout_message.rs"]
mod std_request_timeout_message;

use request_timeout_body::RequestTimeoutBody;
use request_timeout_error::RequestTimeoutError;
pub use request_timeout_layer::RequestTimeoutLayer;
use request_timeout_service::RequestTimeoutService;
use request_timeout_tower_layer::RequestTimeoutTowerLayer;
use std_request_timeout_message::StdRequestTimeoutMessage;

#[cfg(test)]
mod tests {
    #[test]
    fn timeout_layer_preserves_validated_timeout() {
        let timeout =
            super::super::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(1u64))
                .expect("65a8fd30 timeout_layer_preserves_validated_timeout invariant must hold");
        let layer = super::RequestTimeoutLayer::from(timeout);
        assert_eq!(layer.0.get(), std::time::Duration::from_secs(1u64));
    }

    #[tokio::test(start_paused = true)]
    async fn timeout_response_contains_retry_after_without_text_round_trip() {
        let timeout =
            super::super::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(2u64))
                .expect("b140ead4 timeout_response_contains_retry_after_without_text_round_trip invariant must hold");
        let router = axum::Router::from(super::RequestTimeoutLayer::from(timeout).apply(
            super::super::AxumRouter::from(axum::Router::new().route(
                constants_str::VALUE_971BB40E,
                axum::routing::get(async || std::future::pending::<http::StatusCode>().await),
            )),
        ));
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
