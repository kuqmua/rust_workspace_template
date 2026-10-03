#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "owner modules and related behavior retain their intentional facade ordering"
)]
#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_metrics_response_conversion_preserves_utf8_body_and_http_metadata() {
        let text = format!("{}\n{}", constants_str::X, '\u{e9}');
        let response_matches =
            match crate::metrics_response_body::MetricsResponseBody::try_from(text.clone()) {
                Ok(metrics_response_body) => {
                    let response =
                        axum::response::IntoResponse::into_response(metrics_response_body);
                    let metadata_matches = response.status() == http::StatusCode::OK
                        && response
                            .headers()
                            .get(http::header::CONTENT_TYPE)
                            .and_then(|header| header.to_str().ok())
                            .and_then(|header| header.split(';').next())
                            == Some(constants_str::TEXT_PLAIN);
                    metadata_matches
                        && axum::body::to_bytes(response.into_body(), text.len())
                            .await
                            .is_ok_and(|bytes| bytes.as_ref() == text.as_bytes())
                }
                Err(crate::metrics_response_body_error::MetricsResponseBodyError::TooLarge) => {
                    false
                }
            };
        assert!(response_matches);
    }

    async fn call_method(router: axum::Router, method: http::Method) -> http::StatusCode {
        tower::ServiceExt::oneshot(
            router,
            axum::extract::Request::builder()
                .method(method)
                .uri(constants_str::VALUE_C53B39B2)
                .body(axum::body::Body::empty())
                .expect(constants_str::DIAGNOSTIC_49EF0E86),
        )
        .await
        .expect(constants_str::DIAGNOSTIC_12A54113)
        .status()
    }

    #[test]
    fn test_metrics_response_body_is_bounded() {
        let _empty_body =
            crate::metrics_response_body::MetricsResponseBody::try_from(String::new())
                .expect(constants_str::DIAGNOSTIC_52410AD9);
        let exact = String::from_utf8(vec![b'x'; constants_usize::VALUE_8_388_608])
            .expect(constants_str::DIAGNOSTIC_560D1F1E);
        let _exact_body = crate::metrics_response_body::MetricsResponseBody::try_from(exact)
            .expect(constants_str::DIAGNOSTIC_2701B706);
        let _error = crate::metrics_response_body::MetricsResponseBody::try_from(
            String::from_utf8(vec![
                b'x';
                constants_usize::VALUE_8_388_608
                    .saturating_add(constants_usize::ONE)
            ])
            .expect(constants_str::DIAGNOSTIC_329FB604),
        )
        .expect_err(constants_str::F0FC293DD);
    }

    #[test]
    fn test_metrics_response_body_limits_utf8_bytes_and_preserves_text() {
        let exact = '\u{e9}'.to_string().repeat(4_194_304usize);
        let oversized = format!("{exact}{}", constants_str::X);
        assert!(
            crate::metrics_response_body::MetricsResponseBody::try_from(exact.clone())
                .is_ok_and(|body| body.into_inner() == exact)
        );
        assert!(matches!(
            crate::metrics_response_body::MetricsResponseBody::try_from(oversized),
            Err(crate::metrics_response_body_error::MetricsResponseBodyError::TooLarge)
        ));
    }

    #[test]
    fn test_cache_configuration_and_path_text_validate_boundaries() {
        assert_eq!(
            crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum::try_from(constants_usize::ZERO),
            Err(crate::http_metrics_path_cache_maximum_try_from_usize_error::HttpMetricsPathCacheMaximumTryFromUsizeError::Zero)
        );
        assert_eq!(
            crate::http_metrics_path_text::HttpMetricsPathText::try_from(String::new()),
            Err(crate::http_metrics_path_text_error::HttpMetricsPathTextError)
        );
        let _path = crate::http_metrics_path_text::HttpMetricsPathText::try_from(
            constants_str::A_ALT.repeat(constants_usize::VALUE_8_192),
        )
        .expect(constants_str::DIAGNOSTIC_C1B07056);
        assert_eq!(
            crate::http_metrics_path_text::HttpMetricsPathText::try_from(
                constants_str::A_ALT.repeat(8_193usize)
            ),
            Err(crate::http_metrics_path_text_error::HttpMetricsPathTextError)
        );
    }

    #[test]
    fn test_cache_is_bounded_and_reuses_labels() {
        let cache = crate::http_metrics_path_cache::HttpMetricsPathCache::from(
            crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum::from(
                std::num::NonZeroUsize::MIN,
            ),
        );
        assert_eq!(
            cache
                .label(
                    crate::http_metrics_path_text_ref::HttpMetricsPathTextRef::from(
                        constants_str::ROOT
                    )
                )
                .as_str(),
            constants_str::ROOT
        );
        assert_eq!(
            cache
                .label(
                    crate::http_metrics_path_text_ref::HttpMetricsPathTextRef::from(
                        constants_str::ROOT
                    )
                )
                .as_str(),
            constants_str::ROOT
        );
        assert_eq!(
            cache
                .label(
                    crate::http_metrics_path_text_ref::HttpMetricsPathTextRef::from(
                        constants_str::V1
                    )
                )
                .as_str(),
            constants_str::HTTP_METRICS_UNMATCHED_PATH
        );
    }

    #[test]
    fn test_invalid_path_does_not_consume_cache_capacity() {
        let cache = crate::http_metrics_path_cache::HttpMetricsPathCache::from(
            crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum::from(
                std::num::NonZeroUsize::MIN,
            ),
        );
        assert_eq!(
            cache
                .label(
                    crate::http_metrics_path_text_ref::HttpMetricsPathTextRef::from(
                        constants_str::EMPTY
                    )
                )
                .as_str(),
            constants_str::HTTP_METRICS_UNMATCHED_PATH
        );
        assert_eq!(
            cache
                .label(
                    crate::http_metrics_path_text_ref::HttpMetricsPathTextRef::from(
                        constants_str::ROOT
                    )
                )
                .as_str(),
            constants_str::ROOT
        );
    }

    #[test]
    fn test_metrics_path_cache_preserves_utf8_boundaries_and_existing_labels() {
        let cache = crate::http_metrics_path_cache::HttpMetricsPathCache::from(
            crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum::from(
                std::num::NonZeroUsize::MIN,
            ),
        );
        let exact = '\u{e9}'.to_string().repeat(4_096usize);
        let oversized = format!("{exact}{}", constants_str::X);
        assert!(
            crate::http_metrics_path_text::HttpMetricsPathText::try_from(exact.clone())
                .is_ok_and(|path_text| path_text.as_str() == exact)
        );
        assert_eq!(
            cache
                .label(
                    crate::http_metrics_path_text_ref::HttpMetricsPathTextRef::from(
                        oversized.as_str()
                    )
                )
                .as_str(),
            constants_str::HTTP_METRICS_UNMATCHED_PATH
        );
        assert_eq!(
            cache
                .label(
                    crate::http_metrics_path_text_ref::HttpMetricsPathTextRef::from(exact.as_str())
                )
                .as_str(),
            exact
        );
        assert_eq!(
            cache
                .label(
                    crate::http_metrics_path_text_ref::HttpMetricsPathTextRef::from(
                        constants_str::ROOT
                    )
                )
                .as_str(),
            constants_str::HTTP_METRICS_UNMATCHED_PATH
        );
        assert_eq!(
            cache
                .label(
                    crate::http_metrics_path_text_ref::HttpMetricsPathTextRef::from(exact.as_str())
                )
                .as_str(),
            exact
        );
    }

    #[tokio::test]
    async fn test_layer_supports_every_standard_and_custom_http_method() {
        let router = axum::Router::from(
            crate::http_metrics_layer::HttpMetricsLayer::default().apply(
                crate::axum_router::AxumRouter::from(axum::Router::new().route(
                    constants_str::VALUE_B56291E9,
                    axum::routing::any(async || http::StatusCode::INTERNAL_SERVER_ERROR),
                )),
            ),
        );
        let custom = http::Method::from_bytes(b"CUSTOM").expect(constants_str::DIAGNOSTIC_6E90DCA2);
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
