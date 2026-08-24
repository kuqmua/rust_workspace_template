mod request_id;
mod resource_budget;
mod security_headers;
mod service_runtime;

const HTTP_ERROR_EVENT_REQUIRED_FIELD_MASK: u16 = (1u16 << 12u16) - 1u16;
#[derive(optml::Optml, Clone, Debug)]
struct HttpErrorEventCapture {
    error_count: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    field_mask: std::sync::Arc<std::sync::atomic::AtomicU16>,
}
impl<Subscriber> tracing_subscriber::Layer<Subscriber> for HttpErrorEventCapture
where
    Subscriber: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _context: tracing_subscriber::layer::Context<'_, Subscriber>,
    ) {
        if *event.metadata().level() != tracing::Level::ERROR {
            return;
        }
        let _previous_count = self
            .error_count
            .fetch_add(1usize, std::sync::atomic::Ordering::SeqCst);
        let mut visitor = HttpErrorEventFieldVisitor::default();
        event.record(&mut visitor);
        let _previous_mask = self
            .field_mask
            .fetch_or(visitor.mask, std::sync::atomic::Ordering::SeqCst);
    }
}
#[derive(optml::Optml, Debug, Default)]
struct HttpErrorEventFieldVisitor {
    mask: u16,
}
impl HttpErrorEventFieldVisitor {
    fn record_field(&mut self, field: &tracing::field::Field) {
        let bit = match field.name() {
            "request_id" => 1u16 << 0u16,
            "trace_id" => 1u16 << 1u16,
            "service_name" => 1u16 << 2u16,
            "http_route" => 1u16 << 3u16,
            "http_method" => 1u16 << 4u16,
            "http_status" => 1u16 << 5u16,
            "error_code" => 1u16 << 6u16,
            "error_type" => 1u16 << 7u16,
            "error_chain" => 1u16 << 8u16,
            "backtrace" => 1u16 << 9u16,
            "span_trace" => 1u16 << 10u16,
            "error_location" => 1u16 << 11u16,
            _other => 0u16,
        };
        self.mask |= bit;
    }
}
impl tracing::field::Visit for HttpErrorEventFieldVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {
        self.record_field(field);
    }
    fn record_u64(&mut self, field: &tracing::field::Field, _value: u64) {
        self.record_field(field);
    }
}
#[tokio::test(flavor = "current_thread")]
async fn request_span_uses_remote_parent_and_server_kind() {
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );
    let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let tracer =
        opentelemetry::trace::TracerProvider::tracer(&tracer_provider, "server-runtime-test");
    let subscriber = tracing_subscriber::layer::SubscriberExt::with(
        tracing_subscriber::registry(),
        tracing_opentelemetry::layer().with_tracer(tracer),
    );
    let dispatch = tracing::Dispatch::new(subscriber);
    let _dispatch_guard = tracing::dispatcher::set_default(&dispatch);
    let trusted_proxy_ranges = super::TrustedProxyRanges::try_from(vec![
        super::TrustedProxyRange::try_from(str_constants::VALUE_127_0_0_1_32.to_owned())
            .expect("0bb46390 request_span_uses_remote_parent_and_server_kind invariant must hold"),
    ])
    .expect("04cbe253 request_span_uses_remote_parent_and_server_kind invariant must hold");
    let router = axum::Router::from(
        super::RequestIdLayer::with_span_config(super::HttpRequestSpanConfig::new(
            super::ServiceName::from("server-runtime-test"),
            super::StdSocketAddr::from("127.0.0.1:8080".parse::<std::net::SocketAddr>().expect(
                "773561fe request_span_uses_remote_parent_and_server_kind invariant must hold",
            )),
            trusted_proxy_ranges,
        ))
        .apply(super::AxumRouter::from(axum::Router::new().route(
            "/users/{user_id}",
            axum::routing::get(async || http::StatusCode::OK),
        ))),
    );
    let mut request = axum::extract::Request::builder()
        .uri("/users/42")
        .header(
            str_constants::TRACEPARENT,
            str_constants::TRACEPARENT_TEST_VALUE,
        )
        .header(
            str_constants::RUNTIME_FORWARDED_FOR_HEADER_NAME,
            str_constants::VALUE_203_0_113_1,
        )
        .body(axum::body::Body::empty())
        .expect("f56d84cc request_span_uses_remote_parent_and_server_kind invariant must hold");
    let _previous_connect_info = request.extensions_mut().insert(axum::extract::ConnectInfo(
        "127.0.0.1:45000"
            .parse::<std::net::SocketAddr>()
            .expect("0f4a8de7 request_span_uses_remote_parent_and_server_kind invariant must hold"),
    ));
    let response = tower::ServiceExt::oneshot(router, request)
        .await
        .expect("20b587e3 request_span_uses_remote_parent_and_server_kind invariant must hold");
    assert_eq!(response.status(), http::StatusCode::OK);
    drop(response);
    tracer_provider
        .force_flush()
        .expect("8f53d724 request_span_uses_remote_parent_and_server_kind invariant must hold");
    let spans = exporter
        .get_finished_spans()
        .expect("88d108d2 request_span_uses_remote_parent_and_server_kind invariant must hold");
    let request_span = spans
        .iter()
        .find(|span| span.name == "GET /users/{user_id}")
        .expect("fc30b586 request_span_uses_remote_parent_and_server_kind invariant must hold");
    let expected_trace_id = str_constants::TRACEPARENT_TEST_VALUE
        .get(3usize..35usize)
        .expect("34620ae8 request_span_uses_remote_parent_and_server_kind invariant must hold");
    let expected_parent_span_id = str_constants::TRACEPARENT_TEST_VALUE
        .get(36usize..52usize)
        .expect("9c70ecdf request_span_uses_remote_parent_and_server_kind invariant must hold");
    assert_eq!(
        request_span.span_context.trace_id().to_string(),
        expected_trace_id
    );
    assert_eq!(
        request_span.parent_span_id.to_string(),
        expected_parent_span_id
    );
    assert!(request_span.parent_span_is_remote);
    assert_eq!(
        request_span.span_kind,
        opentelemetry::trace::SpanKind::Server
    );
    let attribute = |key| {
        request_span
            .attributes
            .iter()
            .find(|attribute| attribute.key.as_str() == key)
            .map(|attribute| attribute.value.to_string())
    };
    assert_eq!(attribute("http.request.method").as_deref(), Some("GET"));
    assert_eq!(attribute("http.route").as_deref(), Some("/users/{user_id}"));
    assert_eq!(
        attribute(str_constants::OTEL_HTTP_RESPONSE_STATUS_CODE).as_deref(),
        Some("200")
    );
    assert_eq!(
        attribute(str_constants::OTEL_SERVER_ADDRESS).as_deref(),
        Some("127.0.0.1:8080")
    );
    assert_eq!(
        attribute(str_constants::OTEL_CLIENT_ADDRESS).as_deref(),
        Some(str_constants::VALUE_203_0_113_1)
    );
    assert_eq!(
        attribute(str_constants::OTEL_SERVICE_NAME).as_deref(),
        Some("server-runtime-test")
    );
    assert_eq!(
        attribute(str_constants::OTEL_TRACE_ID).as_deref(),
        Some(expected_trace_id)
    );
    assert_eq!(
        attribute(str_constants::OTEL_SPAN_ID).as_deref(),
        Some(request_span.span_context.span_id().to_string().as_str())
    );
    assert_eq!(attribute(str_constants::OTEL_URL_PATH), None);
    tracer_provider
        .shutdown()
        .expect("d478940b request_span_uses_remote_parent_and_server_kind invariant must hold");
}
#[tokio::test(flavor = "current_thread")]
async fn request_span_limits_url_path_and_records_error_telemetry() {
    let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let tracer = opentelemetry::trace::TracerProvider::tracer(&tracer_provider, "http-span-test");
    let subscriber = tracing_subscriber::layer::SubscriberExt::with(
        tracing_subscriber::registry(),
        tracing_opentelemetry::layer().with_tracer(tracer),
    );
    let dispatch = tracing::Dispatch::new(subscriber);
    let _dispatch_guard = tracing::dispatcher::set_default(&dispatch);
    let router = axum::Router::from(super::RequestIdLayer::default().apply(
        super::AxumRouter::from(axum::Router::new().route(
            "/status",
            axum::routing::get(async || {
                let mut response = axum::response::IntoResponse::into_response(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                );
                let _previous = response
                    .extensions_mut()
                    .insert(super::HttpErrorTelemetry::new(
                        super::HttpErrorType::from("persistence.error"),
                        super::HttpErrorCode::from("database_unavailable"),
                    ));
                response
            }),
        )),
    ));
    let status_response = tower::ServiceExt::oneshot(
        router.clone(),
        axum::extract::Request::builder()
            .uri("/status")
            .body(axum::body::Body::empty())
            .expect("bd141981 request_span_limits_url_path_and_records_error_telemetry invariant must hold"),
    )
    .await
    .expect("22fb2978 request_span_limits_url_path_and_records_error_telemetry invariant must hold");
    assert_eq!(
        status_response.status(),
        http::StatusCode::INTERNAL_SERVER_ERROR
    );
    drop(status_response);
    let missing_response = tower::ServiceExt::oneshot(
        router,
        axum::extract::Request::builder()
            .uri("/missing/private-123")
            .body(axum::body::Body::empty())
            .expect("18a1dc0e request_span_limits_url_path_and_records_error_telemetry invariant must hold"),
    )
    .await
    .expect("4dca0c87 request_span_limits_url_path_and_records_error_telemetry invariant must hold");
    assert_eq!(missing_response.status(), http::StatusCode::NOT_FOUND);
    drop(missing_response);
    tracer_provider.force_flush().expect(
        "38b83256 request_span_limits_url_path_and_records_error_telemetry invariant must hold",
    );
    let spans = exporter.get_finished_spans().expect(
        "72d79c7e request_span_limits_url_path_and_records_error_telemetry invariant must hold",
    );
    let status_span = spans.iter().find(|span| span.name == "GET /status").expect(
        "6e0f3748 request_span_limits_url_path_and_records_error_telemetry invariant must hold",
    );
    let status_attribute = |key| {
        status_span
            .attributes
            .iter()
            .find(|attribute| attribute.key.as_str() == key)
            .map(|attribute| attribute.value.to_string())
    };
    assert_eq!(
        status_attribute(str_constants::OTEL_URL_PATH).as_deref(),
        Some("/status")
    );
    assert_eq!(
        status_attribute(str_constants::OTEL_ERROR_TYPE).as_deref(),
        Some("persistence.error")
    );
    assert_eq!(
        status_attribute(str_constants::OTEL_ERROR_CODE).as_deref(),
        Some("database_unavailable")
    );
    let unmatched_span = spans
        .iter()
        .find(|span| span.name == "GET __unmatched__")
        .expect(
            "aa6097d2 request_span_limits_url_path_and_records_error_telemetry invariant must hold",
        );
    assert!(
        unmatched_span
            .attributes
            .iter()
            .all(|attribute| attribute.value.to_string() != "/missing/private-123")
    );
    assert!(
        unmatched_span
            .attributes
            .iter()
            .all(|attribute| attribute.key.as_str() != str_constants::OTEL_URL_PATH)
    );
    let unmatched_attribute = |key| {
        unmatched_span
            .attributes
            .iter()
            .find(|attribute| attribute.key.as_str() == key)
            .map(|attribute| attribute.value.to_string())
    };
    assert_eq!(
        unmatched_attribute(str_constants::OTEL_ERROR_TYPE).as_deref(),
        Some(str_constants::OTEL_HTTP_CLIENT_ERROR_TYPE)
    );
    assert_eq!(
        unmatched_attribute(str_constants::OTEL_ERROR_CODE).as_deref(),
        Some(str_constants::OTEL_HTTP_4XX_ERROR_CODE)
    );
    tracer_provider.shutdown().expect(
        "a4f89d4d request_span_limits_url_path_and_records_error_telemetry invariant must hold",
    );
}
#[tokio::test(flavor = "current_thread")]
async fn http_boundary_emits_one_complete_error_event_only_for_server_errors() {
    #[derive(optml::Optml, Debug, thiserror::Error)]
    #[error("boundary test operation failed")]
    struct BoundaryTestError {
        #[source]
        source: std::io::Error,
    }
    let error_count = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0usize));
    let field_mask = std::sync::Arc::new(std::sync::atomic::AtomicU16::new(0u16));
    let subscriber = tracing_subscriber::layer::SubscriberExt::with(
        tracing_subscriber::registry(),
        HttpErrorEventCapture {
            error_count: std::sync::Arc::clone(&error_count),
            field_mask: std::sync::Arc::clone(&field_mask),
        },
    );
    let dispatch = tracing::Dispatch::new(subscriber);
    let _dispatch_guard = tracing::dispatcher::set_default(&dispatch);
    let expected_diagnostic_line = line!() + 1u32;
    let diagnostic = super::HttpErrorDiagnostic::capture(
        super::HttpErrorTelemetry::new(
            super::HttpErrorType::from("boundary.test"),
            super::HttpErrorCode::from("boundary_failed"),
        ),
        &BoundaryTestError {
            source: std::io::Error::other("nested source"),
        },
    );
    assert!(
        diagnostic
            .error_chain_text()
            .to_string()
            .contains("boundary test operation failed: nested source")
    );
    assert!(!diagnostic.backtrace().to_string().is_empty());
    assert!(!diagnostic.span_trace().to_string().is_empty());
    assert!(
        diagnostic
            .location()
            .to_string()
            .contains(expected_diagnostic_line.to_string().as_str())
    );
    let server_error_diagnostic = diagnostic.clone();
    let router = axum::Router::from(
        super::RequestIdLayer::with_span_config(super::HttpRequestSpanConfig::new(
            super::ServiceName::from("boundary-test"),
            super::StdSocketAddr::from(
                "127.0.0.1:8080"
                    .parse::<std::net::SocketAddr>()
                    .expect("c74109ca http_boundary_emits_one_complete_error_event_only_for_server_errors invariant must hold"),
            ),
            super::TrustedProxyRanges::default(),
        ))
        .apply(super::AxumRouter::from(
            axum::Router::new()
                .route(
                    "/failure",
                    axum::routing::get(move || {
                        let response_diagnostic = server_error_diagnostic.clone();
                        async move {
                            let mut response = axum::response::IntoResponse::into_response(
                                http::StatusCode::INTERNAL_SERVER_ERROR,
                            );
                            let _previous = response.extensions_mut().insert(response_diagnostic);
                            response
                        }
                    }),
                )
                .route(
                    "/invalid",
                    axum::routing::get(async || http::StatusCode::UNPROCESSABLE_ENTITY),
                ),
        )),
    );
    let server_error_response = tower::ServiceExt::oneshot(
        router.clone(),
        axum::extract::Request::builder()
            .uri("/failure")
            .body(axum::body::Body::empty())
            .expect("2b710c82 http_boundary_emits_one_complete_error_event_only_for_server_errors invariant must hold"),
    )
    .await
    .expect("33c72c1c http_boundary_emits_one_complete_error_event_only_for_server_errors invariant must hold");
    assert_eq!(
        server_error_response.status(),
        http::StatusCode::INTERNAL_SERVER_ERROR
    );
    drop(server_error_response);
    assert_eq!(
        error_count.load(std::sync::atomic::Ordering::SeqCst),
        1usize
    );
    assert_eq!(
        field_mask.load(std::sync::atomic::Ordering::SeqCst),
        HTTP_ERROR_EVENT_REQUIRED_FIELD_MASK
    );
    let client_error_response = tower::ServiceExt::oneshot(
        router,
        axum::extract::Request::builder()
            .uri("/invalid")
            .body(axum::body::Body::empty())
            .expect("b362c5d1 http_boundary_emits_one_complete_error_event_only_for_server_errors invariant must hold"),
    )
    .await
    .expect("e271f216 http_boundary_emits_one_complete_error_event_only_for_server_errors invariant must hold");
    assert_eq!(
        client_error_response.status(),
        http::StatusCode::UNPROCESSABLE_ENTITY
    );
    drop(client_error_response);
    assert_eq!(
        error_count.load(std::sync::atomic::Ordering::SeqCst),
        1usize
    );
}
#[test]
fn observed_client_preparation_injects_context_and_creates_child_span() {
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_sdk::propagation::TraceContextPropagator::new(),
    );
    let exporter = opentelemetry_sdk::trace::InMemorySpanExporterBuilder::new().build();
    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(exporter.clone())
        .build();
    let tracer = opentelemetry::trace::TracerProvider::tracer(
        &tracer_provider,
        "server-runtime-client-test",
    );
    let subscriber = tracing_subscriber::layer::SubscriberExt::with(
        tracing_subscriber::registry(),
        tracing_opentelemetry::layer().with_tracer(tracer),
    );
    let dispatch = tracing::Dispatch::new(subscriber);
    let _dispatch_guard = tracing::dispatcher::set_default(&dispatch);
    let url = reqwest::Url::parse(str_constants::HTTPS_EXAMPLE_COM).expect("a0c9b8a8 observed_client_preparation_injects_context_and_creates_child_span invariant must hold");
    let mut request = super::ReqwestRequest::from(reqwest::Request::new(http::Method::GET, url));
    let root_span = tracing::info_span!("caller");
    let prepared_client_span =
        root_span.in_scope(|| super::ReqwestClient::prepare_observed_http_request(&mut request));
    let prepared_request = request.into_inner();
    assert!(
        prepared_request
            .headers()
            .get(str_constants::TRACEPARENT)
            .is_some()
    );
    drop(prepared_client_span);
    drop(root_span);
    let spans = exporter.get_finished_spans().expect("a472015a observed_client_preparation_injects_context_and_creates_child_span invariant must hold");
    let caller_span = spans
        .iter()
        .find(|span| span.name == "caller")
        .expect("87c0e547 observed_client_preparation_injects_context_and_creates_child_span invariant must hold");
    let exported_client_span = spans
        .iter()
        .find(|span| span.name == "GET example.com")
        .expect("5bfcb617 observed_client_preparation_injects_context_and_creates_child_span invariant must hold");
    assert_eq!(
        exported_client_span.span_context.trace_id(),
        caller_span.span_context.trace_id()
    );
    assert_eq!(
        exported_client_span.parent_span_id,
        caller_span.span_context.span_id()
    );
    assert_eq!(
        exported_client_span.span_kind,
        opentelemetry::trace::SpanKind::Client
    );
    tracer_provider.shutdown().expect("721ff26e observed_client_preparation_injects_context_and_creates_child_span invariant must hold");
}
