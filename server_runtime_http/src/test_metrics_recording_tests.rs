#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_deref_inner::DerefInner,
)]
struct MetricsTestKey(metrics::Key);

#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_deref_inner::DerefInner,
)]
struct MetricsTestCounterArc(std::sync::Arc<metrics::atomics::AtomicU64>);

impl metrics::HistogramFn for MetricsTestCounterArc {
    fn record(&self, _f64: f64) {
        let _previous = self.fetch_add(1u64, std::sync::atomic::Ordering::Release);
    }
}

#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_new::New,
    proc_macro_getters::Getters,
)]
#[getters(bare)]
struct MetricsTestRecorder {
    #[getters(skip)]
    request_key: MetricsTestKey,
    #[getters(skip)]
    error_key: MetricsTestKey,
    #[getters(skip)]
    histogram_key: MetricsTestKey,
    requests: MetricsTestCounterArc,
    errors: MetricsTestCounterArc,
    histogram_samples: MetricsTestCounterArc,
}

impl From<MetricsTestKey> for MetricsTestRecorder {
    fn from(value: MetricsTestKey) -> Self {
        let labels = value.labels().cloned().collect::<Vec<_>>();
        let counter = || {
            MetricsTestCounterArc::from(std::sync::Arc::new(metrics::atomics::AtomicU64::new(0u64)))
        };
        Self::new(
            value,
            MetricsTestKey::from(metrics::Key::from_parts(
                constants_str::HTTP_METRICS_ERRORS_TOTAL,
                labels.iter(),
            )),
            MetricsTestKey::from(metrics::Key::from_parts(
                constants_str::HTTP_METRICS_REQUEST_DURATION_SECONDS,
                labels.iter(),
            )),
            counter(),
            counter(),
            counter(),
        )
    }
}

impl metrics::Recorder for MetricsTestRecorder {
    fn describe_counter(
        &self,
        key_name: metrics::KeyName,
        unit: Option<metrics::Unit>,
        shared_string: metrics::SharedString,
    ) {
        drop((key_name, unit, shared_string));
    }
    fn describe_gauge(
        &self,
        key_name: metrics::KeyName,
        unit: Option<metrics::Unit>,
        shared_string: metrics::SharedString,
    ) {
        drop((key_name, unit, shared_string));
    }
    fn describe_histogram(
        &self,
        key_name: metrics::KeyName,
        unit: Option<metrics::Unit>,
        shared_string: metrics::SharedString,
    ) {
        drop((key_name, unit, shared_string));
    }
    fn register_counter(
        &self,
        key: &metrics::Key,
        _metadata: &metrics::Metadata<'_>,
    ) -> metrics::Counter {
        if key == &*self.request_key {
            metrics::Counter::from_arc(std::sync::Arc::clone(&self.requests))
        } else {
            assert_eq!(key, &*self.error_key);
            metrics::Counter::from_arc(std::sync::Arc::clone(&self.errors))
        }
    }
    fn register_gauge(
        &self,
        _key: &metrics::Key,
        _metadata: &metrics::Metadata<'_>,
    ) -> metrics::Gauge {
        metrics::Gauge::noop()
    }
    fn register_histogram(
        &self,
        key: &metrics::Key,
        _metadata: &metrics::Metadata<'_>,
    ) -> metrics::Histogram {
        assert_eq!(key, &*self.histogram_key);
        metrics::Histogram::from_arc(std::sync::Arc::new(self.histogram_samples.clone()))
    }
}

#[tokio::test]
async fn test_metrics_record_exact_method_path_status_labels_and_counter_increments() {
    let check = async |method, status, method_label, uri, expected_path, matched| {
        let labels = [
            metrics::Label::new(constants_str::HTTP_METRICS_LABEL_METHOD, method_label),
            metrics::Label::new(constants_str::PATH_ALT_5, expected_path),
            metrics::Label::new(
                constants_str::STATUS_ALT,
                http::StatusCode::as_str(&status).to_owned(),
            ),
        ];
        let recorder = MetricsTestRecorder::from(MetricsTestKey::from(metrics::Key::from_parts(
            constants_str::HTTP_METRICS_REQUESTS_TOTAL,
            labels.iter(),
        )));
        let inner_router = if matched {
            axum::Router::new().route(
                constants_str::VALUE_B56291E9,
                axum::routing::any(async move || status),
            )
        } else {
            axum::Router::new().fallback(async move || status)
        };
        let router = axum::Router::from(
            crate::http_metrics_layer::HttpMetricsLayer::default()
                .apply(crate::axum_router::AxumRouter::from(inner_router)),
        );
        let request_result = axum::extract::Request::builder()
            .method(method)
            .uri(uri)
            .body(axum::body::Body::empty());
        assert!(request_result.is_ok());
        let guard = metrics::set_default_local_recorder(&recorder);
        let response_option = match request_result {
            Ok(request) => tower::ServiceExt::oneshot(router, request).await.ok(),
            Err(_error) => None,
        };
        drop(guard);
        assert!(response_option.is_some_and(|response| response.status() == status));
        assert_eq!(
            recorder
                .requests()
                .load(std::sync::atomic::Ordering::Acquire),
            1u64
        );
        assert_eq!(
            recorder.errors().load(std::sync::atomic::Ordering::Acquire),
            u64::from(status.is_server_error())
        );
        assert_eq!(
            recorder
                .histogram_samples()
                .load(std::sync::atomic::Ordering::Acquire),
            1u64
        );
    };
    check(
        http::Method::GET,
        http::StatusCode::OK,
        constants_str::GET,
        constants_str::VALUE_C53B39B2,
        constants_str::VALUE_B56291E9,
        true,
    )
    .await;
    check(
        http::Method::POST,
        http::StatusCode::BAD_REQUEST,
        constants_str::POST,
        constants_str::VALUE_C53B39B2,
        constants_str::VALUE_B56291E9,
        true,
    )
    .await;
    check(
        http::Method::DELETE,
        http::StatusCode::INTERNAL_SERVER_ERROR,
        constants_str::DELETE,
        constants_str::VALUE_C53B39B2,
        constants_str::VALUE_B56291E9,
        true,
    )
    .await;
    check(
        http::Method::CONNECT,
        http::StatusCode::OK,
        constants_str::HTTP_METHOD_CONNECT_LABEL,
        constants_str::VALUE_C53B39B2,
        constants_str::VALUE_B56291E9,
        true,
    )
    .await;
    check(
        http::Method::HEAD,
        http::StatusCode::OK,
        constants_str::HTTP_METHOD_HEAD_LABEL,
        constants_str::VALUE_C53B39B2,
        constants_str::VALUE_B56291E9,
        true,
    )
    .await;
    check(
        http::Method::OPTIONS,
        http::StatusCode::OK,
        constants_str::HTTP_METHOD_OPTIONS_LABEL,
        constants_str::VALUE_C53B39B2,
        constants_str::VALUE_B56291E9,
        true,
    )
    .await;
    check(
        http::Method::PATCH,
        http::StatusCode::OK,
        constants_str::PATCH,
        constants_str::VALUE_C53B39B2,
        constants_str::VALUE_B56291E9,
        true,
    )
    .await;
    check(
        http::Method::PUT,
        http::StatusCode::OK,
        constants_str::HTTP_METHOD_PUT_LABEL,
        constants_str::VALUE_C53B39B2,
        constants_str::VALUE_B56291E9,
        true,
    )
    .await;
    check(
        http::Method::TRACE,
        http::StatusCode::OK,
        constants_str::HTTP_METHOD_TRACE_LABEL,
        constants_str::VALUE_C53B39B2,
        constants_str::VALUE_B56291E9,
        true,
    )
    .await;
    let custom_result = http::Method::from_bytes(constants_str::X.as_bytes());
    assert!(custom_result.is_ok());
    if let Ok(custom) = custom_result {
        check(
            custom,
            http::StatusCode::OK,
            constants_str::HTTP_METHOD_OTHER_LABEL,
            constants_str::VALUE_C53B39B2,
            constants_str::VALUE_B56291E9,
            true,
        )
        .await;
    }
    check(
        http::Method::GET,
        http::StatusCode::OK,
        constants_str::GET,
        constants_str::TEST_DYNAMIC_IDENTIFIER_PATH,
        constants_str::TEST_NORMALIZED_IDENTIFIER_PATH,
        false,
    )
    .await;
    check(
        http::Method::GET,
        http::StatusCode::NOT_FOUND,
        constants_str::GET,
        constants_str::ROOT,
        constants_str::HTTP_METRICS_UNMATCHED_PATH,
        false,
    )
    .await;
}

#[tokio::test]
async fn test_metrics_inner_service_error_preserves_source_without_emitting_response_metrics() {
    let recorder = MetricsTestRecorder::from(MetricsTestKey::from(metrics::Key::from_name(
        constants_str::HTTP_METRICS_REQUESTS_TOTAL,
    )));
    let expected = std::io::Error::new(std::io::ErrorKind::BrokenPipe, constants_str::X);
    let inner = tower::service_fn(move |_request: axum::extract::Request| {
        std::future::ready(Err::<axum::response::Response, _>(
            crate::bounded_read_io_error::BoundedReadIoError::from(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                constants_str::X,
            )),
        ))
    });
    let paths = crate::shared_http_metrics_path_cache_arc::SharedHttpMetricsPathCacheArc::from(
        crate::http_metrics_path_cache::HttpMetricsPathCache::from(
            crate::http_metrics_path_cache_maximum::HttpMetricsPathCacheMaximum::from(
                std::num::NonZeroUsize::MIN,
            ),
        ),
    );
    let service = crate::http_metrics_service::HttpMetricsService::new(inner, paths);
    let request = axum::extract::Request::new(axum::body::Body::empty());
    let guard = metrics::set_default_local_recorder(&recorder);
    let result = tower::ServiceExt::oneshot(service, request).await;
    drop(guard);
    assert!(result.is_err_and(|error| {
        error.kind() == expected.kind()
            && error.to_string() == expected.to_string()
            && error
                .get_ref()
                .zip(expected.get_ref())
                .is_some_and(|(source, expected_source)| {
                    source.to_string() == expected_source.to_string()
                })
    }));
    assert_eq!(
        recorder
            .requests()
            .load(std::sync::atomic::Ordering::Acquire),
        0u64
    );
    assert_eq!(
        recorder.errors().load(std::sync::atomic::Ordering::Acquire),
        0u64
    );
    assert_eq!(
        recorder
            .histogram_samples()
            .load(std::sync::atomic::Ordering::Acquire),
        0u64
    );
}
