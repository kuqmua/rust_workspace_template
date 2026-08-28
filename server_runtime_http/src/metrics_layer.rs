#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules and related behavior retain their intentional facade ordering"
)]

pub use crate::http_metrics_layer::HttpMetricsLayer;
use crate::http_metrics_path_cache::HttpMetricsPathCache;
pub use crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum;
use crate::http_metrics_path_cache_maximum_non_zero_usize::HttpMetricsPathCacheMaximumNonZeroUsize;
pub use crate::http_metrics_path_cache_maximum_try_from_usize_error::HttpMetricsPathCacheMaximumTryFromUsizeError;
use crate::http_metrics_path_entries_rw_lock::HttpMetricsPathEntriesRwLock;
use crate::http_metrics_path_text::HttpMetricsPathText;
use crate::http_metrics_path_text_error::HttpMetricsPathTextError;
use crate::http_metrics_path_text_ref::HttpMetricsPathTextRef;
use crate::http_metrics_service::HttpMetricsService;
use crate::http_metrics_tower_layer::HttpMetricsTowerLayer;
pub use crate::metrics_response_body::MetricsResponseBody;
pub use crate::metrics_response_body_error::MetricsResponseBodyError;
use crate::metrics_shared_string::MetricsSharedString;
use crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc;

#[cfg(test)]
mod tests {
    async fn call_method(router: axum::Router, method: http::Method) -> http::StatusCode {
        tower::ServiceExt::oneshot(
            router,
            axum::extract::Request::builder()
                .method(method)
                .uri(constants_str::VALUE_C53B39B2)
                .body(axum::body::Body::empty())
                .expect("49ef0e86 call_method invariant must hold"),
        )
        .await
        .expect("12a54113 call_method invariant must hold")
        .status()
    }

    #[test]
    fn metrics_response_body_is_bounded() {
        let _empty_body = super::MetricsResponseBody::try_from(String::new())
            .expect("52410ad9 metrics_response_body_is_bounded invariant must hold");
        let exact = String::from_utf8(vec![b'x'; constants_usize::VALUE_8_388_608])
            .expect("560d1f1e metrics_response_body_is_bounded invariant must hold");
        let _exact_body = super::MetricsResponseBody::try_from(exact)
            .expect("2701b706 metrics_response_body_is_bounded invariant must hold");
        let _error = super::MetricsResponseBody::try_from(
            String::from_utf8(vec![
                b'x';
                constants_usize::VALUE_8_388_608
                    .saturating_add(constants_usize::ONE)
            ])
            .expect("329fb604 metrics_response_body_is_bounded invariant must hold"),
        )
        .expect_err(constants_str::F0FC293DD);
    }

    #[test]
    fn cache_configuration_and_path_text_validate_boundaries() {
        assert_eq!(
            super::HttpMetricsPathCacheMaximum::try_from(constants_usize::ZERO),
            Err(super::HttpMetricsPathCacheMaximumTryFromUsizeError)
        );
        assert_eq!(
            super::HttpMetricsPathText::try_from(String::new()),
            Err(super::HttpMetricsPathTextError)
        );
        let _path = super::HttpMetricsPathText::try_from(
            constants_str::A_ALT.repeat(constants_usize::VALUE_8_192),
        )
        .expect(
            "c1b07056 cache_configuration_and_path_text_validate_boundaries invariant must hold",
        );
        assert_eq!(
            super::HttpMetricsPathText::try_from("a".repeat(8_193usize)),
            Err(super::HttpMetricsPathTextError)
        );
    }

    #[test]
    fn cache_is_bounded_and_reuses_labels() {
        let cache = super::HttpMetricsPathCache::new(super::HttpMetricsPathCacheMaximum::from(
            std::num::NonZeroUsize::MIN,
        ));
        assert_eq!(
            cache
                .label(super::HttpMetricsPathTextRef::from(constants_str::ROOT))
                .0
                .as_ref(),
            constants_str::ROOT
        );
        assert_eq!(
            cache
                .label(super::HttpMetricsPathTextRef::from(constants_str::ROOT))
                .0
                .as_ref(),
            constants_str::ROOT
        );
        assert_eq!(
            cache
                .label(super::HttpMetricsPathTextRef::from(constants_str::V1))
                .0
                .as_ref(),
            constants_str::HTTP_METRICS_UNMATCHED_PATH
        );
    }

    #[test]
    fn invalid_path_does_not_consume_cache_capacity() {
        let cache = super::HttpMetricsPathCache::new(super::HttpMetricsPathCacheMaximum::from(
            std::num::NonZeroUsize::MIN,
        ));
        assert_eq!(
            cache
                .label(super::HttpMetricsPathTextRef::from(constants_str::EMPTY))
                .0
                .as_ref(),
            constants_str::HTTP_METRICS_UNMATCHED_PATH
        );
        assert_eq!(
            cache
                .label(super::HttpMetricsPathTextRef::from(constants_str::ROOT))
                .0
                .as_ref(),
            constants_str::ROOT
        );
    }

    #[tokio::test]
    async fn layer_supports_every_standard_and_custom_http_method() {
        let router = axum::Router::from(super::HttpMetricsLayer::default().apply(
            crate::domain_types::AxumRouter::from(axum::Router::new().route(
                constants_str::VALUE_B56291E9,
                axum::routing::any(async || http::StatusCode::INTERNAL_SERVER_ERROR),
            )),
        ));
        let custom = http::Method::from_bytes(b"CUSTOM").expect(
            "6e90dca2 layer_supports_every_standard_and_custom_http_method invariant must hold",
        );
        let statuses = tokio::join!(
            call_method(router.clone(), http::Method::CONNECT),
            call_method(router.clone(), http::Method::DELETE),
            call_method(router.clone(), http::Method::GET),
            call_method(router.clone(), http::Method::HEAD),
            call_method(router.clone(), http::Method::OPTIONS),
            call_method(router.clone(), http::Method::PATCH),
            call_method(router.clone(), http::Method::POST),
            call_method(router.clone(), http::Method::PUT),
            call_method(router.clone(), http::Method::TRACE),
            call_method(router, custom),
        );
        assert_eq!(
            statuses,
            (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
                http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        );
    }
}

// Root-owned module compatibility wrappers.
mod http_metrics_layer {
    pub use crate::http_metrics_layer::*;
}
mod http_metrics_path_cache {
    pub use crate::http_metrics_path_cache::*;
}
mod http_metrics_path_cache_maximum {
    pub use crate::http_metrics_path_cache_maximum::*;
}
mod http_metrics_path_cache_maximum_non_zero_usize {
    pub use crate::http_metrics_path_cache_maximum_non_zero_usize::*;
}
mod http_metrics_path_cache_maximum_try_from_usize_error {
    pub use crate::http_metrics_path_cache_maximum_try_from_usize_error::*;
}
mod http_metrics_path_entries_rw_lock {
    pub use crate::http_metrics_path_entries_rw_lock::*;
}
mod http_metrics_path_text {
    pub use crate::http_metrics_path_text::*;
}
mod http_metrics_path_text_error {
    pub use crate::http_metrics_path_text_error::*;
}
mod http_metrics_path_text_ref {
    pub use crate::http_metrics_path_text_ref::*;
}
mod http_metrics_service {
    pub use crate::http_metrics_service::*;
}
mod http_metrics_tower_layer {
    pub use crate::http_metrics_tower_layer::*;
}
mod metrics_response_body {
    pub use crate::metrics_response_body::*;
}
mod metrics_response_body_error {
    pub use crate::metrics_response_body_error::*;
}
mod metrics_shared_string {
    pub use crate::metrics_shared_string::*;
}
mod shared_http_metrics_path_cache_arc {
    pub use crate::shared_http_metrics_path_cache_arc::*;
}
