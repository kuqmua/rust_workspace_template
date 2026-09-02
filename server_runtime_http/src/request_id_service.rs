#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, proc_macro_new::New,
)]
#[constructor(pub(crate))]
pub(super) struct RequestIdService<Service> {
    inner: Service,
    span_config: Option<super::http_request_span_config::HttpRequestSpanConfig>,
}
impl<Service> tower::Service<axum::extract::Request> for RequestIdService<Service>
where
    Service: tower::Service<axum::extract::Request, Response = axum::response::Response>
        + Send
        + 'static,
    Service::Future: Send + 'static,
{
    type Error = Service::Error;
    type Future = std::pin::Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>,
    >;
    type Response = axum::response::Response;
    fn call(&mut self, mut req: axum::extract::Request) -> Self::Future {
        let remote_context = crate::extract_remote_trace_context::extract_remote_trace_context(
            crate::http_opentelemetry_header_map_ref::HttpOpentelemetryHeaderMapRef::from(
                req.headers(),
            ),
        );
        let request_id_and_header_value = [
            constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID,
            constants_str::RUNTIME_CORRELATION_ID_HEADER_NAME,
        ]
        .into_iter()
        .find_map(|header_name| {
            req.headers().get(header_name).and_then(|value| {
                crate::request_id::RequestId::try_from(value)
                    .ok()
                    .map(|request_id| (request_id, value.clone()))
            })
        })
        .unwrap_or_else(|| {
            loop {
                if let Ok(value) =
                    crate::request_id::RequestId::try_from(uuid::Uuid::new_v4().to_string())
                    && let Ok(header_value) = http::HeaderValue::try_from(&value)
                {
                    break (value, header_value);
                }
            }
        });
        let started_at = tokio::time::Instant::now();
        let matched_route = req
            .extensions()
            .get::<axum::extract::MatchedPath>()
            .map(axum::extract::MatchedPath::as_str);
        let route = matched_route.unwrap_or(constants_str::HTTP_METRICS_UNMATCHED_PATH);
        let safe_url_path = matched_route.filter(|matched_path| {
            !matched_path.contains('{') && *matched_path == req.uri().path()
        });
        let client_address = self.span_config.as_ref().and_then(|span_config| {
            req.extensions()
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|connect_info| {
                    crate::resolve_client_ip::resolve_client_ip(
                        crate::http_header_map_ref::HttpHeaderMapRef::from(req.headers()),
                        crate::client_socket_addr::ClientSocketAddr::from(connect_info.0),
                        span_config.trusted_proxy_ranges(),
                    )
                })
        });
        let span = tracing::info_span!(
            "http.request",
            otel.kind = "server",
            otel.name = tracing::field::Empty,
            otel.status_code = tracing::field::Empty,
            request_id = %request_id_and_header_value.0,
            "http.request.method" = %req.method(),
            "http.route" = %route,
            "http.response.status_code" = tracing::field::Empty,
            "url.path" = tracing::field::Empty,
            "error.type" = tracing::field::Empty,
            error_code = tracing::field::Empty,
            "server.address" = tracing::field::Empty,
            "client.address" = tracing::field::Empty,
            trace_id = tracing::field::Empty,
            span_id = tracing::field::Empty,
            "service.name" = tracing::field::Empty,
        );
        let _server_name_record = span.record(
            constants_str::OTEL_NAME,
            format_args!("{} {route}", req.method()),
        );
        if let Some(path) = safe_url_path {
            let _url_path_record = span.record(constants_str::OTEL_URL_PATH, path);
        }
        if let Some(span_config) = &self.span_config {
            let _server_address_record = span.record(
                constants_str::OTEL_SERVER_ADDRESS,
                tracing::field::display(span_config.server_address()),
            );
            let _service_name_record = span.record(
                constants_str::OTEL_SERVICE_NAME,
                tracing::field::display(span_config.service_name()),
            );
        }
        if let Some(address) = client_address {
            let _client_address_record = span.record(
                constants_str::OTEL_CLIENT_ADDRESS,
                tracing::field::display(address),
            );
        }
        if let Err(error) = tracing_opentelemetry::OpenTelemetrySpanExt::set_parent(
            &span,
            (*remote_context).clone(),
        ) {
            tracing::warn!(
                error = %error,
                message = %constants_str::TRACING_HTTP_REMOTE_PARENT_ATTACH_FAILED,
            );
        }
        let span_context = tracing_opentelemetry::OpenTelemetrySpanExt::context(&span);
        let opentelemetry_span = opentelemetry::trace::TraceContextExt::span(&span_context);
        let trace_id = opentelemetry_span.span_context().trace_id().to_string();
        let span_id = opentelemetry_span.span_context().span_id().to_string();
        let request_id = request_id_and_header_value.0.clone();
        let http_method = req.method().clone();
        let http_route = route.to_owned();
        let service_name = self
            .span_config
            .as_ref()
            .map_or_else(String::new, |config| config.service_name().to_string());
        let _trace_id_record = span.record(constants_str::OTEL_TRACE_ID, trace_id.as_str());
        let _span_id_record = span.record(constants_str::OTEL_SPAN_ID, span_id.as_str());
        let _previous_extension_request_id =
            req.extensions_mut().insert(request_id_and_header_value.0);
        let response_future = tower::Service::call(&mut self.inner, req);
        Box::pin(tracing::Instrument::instrument(
            async move {
                let mut response = response_future.await?;
                let _server_status_record = tracing::Span::current().record(
                    constants_str::OTEL_HTTP_RESPONSE_STATUS_CODE,
                    response.status().as_u16(),
                );
                if response.status().is_server_error() {
                    let _server_error_record = tracing::Span::current().record(
                        constants_str::OTEL_STATUS_CODE,
                        constants_str::OTEL_ERROR_STATUS,
                    );
                }
                if response.status().is_client_error() || response.status().is_server_error() {
                    let default_error_telemetry = if response.status().is_server_error() {
                        crate::http_error_telemetry::HttpErrorTelemetry::new(
                            crate::http_error_type::HttpErrorType::from(
                                constants_str::OTEL_HTTP_SERVER_ERROR_TYPE,
                            ),
                            crate::http_error_code::HttpErrorCode::from(
                                constants_str::OTEL_HTTP_5XX_ERROR_CODE,
                            ),
                        )
                    } else {
                        crate::http_error_telemetry::HttpErrorTelemetry::new(
                            crate::http_error_type::HttpErrorType::from(
                                constants_str::OTEL_HTTP_CLIENT_ERROR_TYPE,
                            ),
                            crate::http_error_code::HttpErrorCode::from(
                                constants_str::OTEL_HTTP_4XX_ERROR_CODE,
                            ),
                        )
                    };
                    let optional_diagnostic = response
                        .extensions()
                        .get::<crate::http_error_diagnostic::HttpErrorDiagnostic>(
                    );
                    let error_telemetry = optional_diagnostic
                        .map(crate::http_error_telemetry::HttpErrorTelemetry::from)
                        .or_else(|| {
                            response
                                .extensions()
                                .get::<crate::http_error_telemetry::HttpErrorTelemetry>()
                                .copied()
                        })
                        .unwrap_or(default_error_telemetry);
                    let _error_type_record = tracing::Span::current().record(
                        constants_str::OTEL_ERROR_TYPE,
                        tracing::field::display(error_telemetry.error_type()),
                    );
                    let _error_code_record = tracing::Span::current().record(
                        constants_str::OTEL_ERROR_CODE,
                        tracing::field::display(error_telemetry.error_code()),
                    );
                    if response.status().is_server_error() {
                        let mut fallback_diagnostic = None;
                        let diagnostic = optional_diagnostic.map_or_else(
                            || {
                                &*fallback_diagnostic.insert(crate::http_error_diagnostic::HttpErrorDiagnostic::capture(
                                    error_telemetry,
                                    &crate::http_error_without_diagnostic_context::HttpErrorWithoutDiagnosticContext::Missing,
                                ))
                            },
                            |diagnostic| diagnostic,
                        );
                        tracing::error!(
                            request_id = %request_id,
                            trace_id = %trace_id,
                            service_name = %service_name,
                            http_route = %http_route,
                            http_method = %http_method,
                            http_status = response.status().as_u16(),
                            error_code = %error_telemetry.error_code(),
                            error_type = %error_telemetry.error_type(),
                            error_chain = %diagnostic.error_chain_text(),
                            error_location = %diagnostic.location(),
                            backtrace = %diagnostic.backtrace(),
                            span_trace = %diagnostic.span_trace(),
                            duration_ms = started_at.elapsed().as_millis(),
                            message = %constants_str::HTTP_REQUEST_FAILED,
                        );
                    }
                }
                if !response.status().is_server_error() {
                    tracing::info!(
                        status = response.status().as_u16(),
                        duration_ms = started_at.elapsed().as_millis(),
                        message = %constants_str::TRACING_HTTP_REQUEST_COMPLETED,
                    );
                }
                let _previous_header_request_id = response.headers_mut().insert(
                    http::HeaderName::from_static(constants_str::HTTP_HEADER_NAMES_X_REQUEST_ID),
                    request_id_and_header_value.1.clone(),
                );
                let _previous_correlation_id = response.headers_mut().insert(
                    http::HeaderName::from_static(
                        constants_str::RUNTIME_CORRELATION_ID_HEADER_NAME,
                    ),
                    request_id_and_header_value.1,
                );
                Ok(response)
            },
            span,
        ))
    }
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
}
