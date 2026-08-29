use super::request_timeout_body::RequestTimeoutBody;
use super::request_timeout_error::RequestTimeoutError;
pub use super::request_timeout_layer::RequestTimeoutLayer;
use super::request_timeout_service::RequestTimeoutService;
use super::request_timeout_tower_layer::RequestTimeoutTowerLayer;
use super::std_request_timeout_message::StdRequestTimeoutMessage;
#[cfg(test)]
mod tests {
    #[test]
    fn timeout_layer_preserves_validated_timeout() {
        let timeout = crate::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(1u64))
            .expect("65a8fd30 timeout_layer_preserves_validated_timeout invariant must hold");
        let layer = super::RequestTimeoutLayer::from(timeout);
        assert_eq!(layer.0.get(), std::time::Duration::from_secs(1u64));
    }

    #[tokio::test(start_paused = true)]
    async fn timeout_response_contains_retry_after_without_text_round_trip() {
        let timeout =
            crate::RequestTimeoutDuration::try_from(std::time::Duration::from_secs(2u64))
                .expect("b140ead4 timeout_response_contains_retry_after_without_text_round_trip invariant must hold");
        let router = axum::Router::from(super::RequestTimeoutLayer::from(timeout).apply(
            crate::AxumRouter::from(axum::Router::new().route(
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

// Root-owned module compatibility wrappers.
mod request_timeout_body {
    pub use super::super::request_timeout_body::*;
}
mod request_timeout_error {
    pub use super::super::request_timeout_error::*;
}
mod request_timeout_layer {
    pub use super::super::request_timeout_layer::*;
}
mod request_timeout_service {
    pub use super::super::request_timeout_service::*;
}
mod request_timeout_tower_layer {
    pub use super::super::request_timeout_tower_layer::*;
}
mod std_request_timeout_message {
    pub use super::super::std_request_timeout_message::*;
}
