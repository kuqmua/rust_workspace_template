#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct ReqwestClient(reqwest::Client);
impl ReqwestClient {
    pub async fn execute(
        &self,
        mut request: crate::reqwest_request::ReqwestRequest,
    ) -> Result<crate::reqwest_response::ReqwestResponse, crate::reqwest_error::ReqwestError> {
        let span = Self::prepare_observed_http_request(&mut request);
        tracing::Instrument::instrument(
            async {
                match self.0.execute(request.into_inner()).await {
                    Ok(response) => {
                        let _client_status_record = tracing::Span::current().record(
                            constants_str::test_fixtures::OTEL_HTTP_RESPONSE_STATUS_CODE,
                            response.status().as_u16(),
                        );
                        if response.status().is_server_error() {
                            let _client_error_record = tracing::Span::current().record(
                                constants_str::test_fixtures::OTEL_STATUS_CODE,
                                constants_str::test_fixtures::OTEL_ERROR_STATUS,
                            );
                        }
                        Ok(crate::reqwest_response::ReqwestResponse::from(response))
                    }
                    Err(error) => {
                        let _client_error_record = tracing::Span::current().record(
                            constants_str::test_fixtures::OTEL_STATUS_CODE,
                            constants_str::test_fixtures::OTEL_ERROR_STATUS,
                        );
                        Err(crate::reqwest_error::ReqwestError::from(error))
                    }
                }
            },
            span.into_inner(),
        )
        .await
    }

    #[allow(
        clippy::single_call_fn,
        reason = "request instrumentation preparation is shared with deterministic tests"
    )]
    pub(crate) fn prepare_observed_http_request(
        request: &mut crate::reqwest_request::ReqwestRequest,
    ) -> super::tracing_http_client_span::TracingHttpClientSpan {
        let span = {
            let method = request.method();
            let host = request.host().unwrap_or_else(|| {
                crate::http_host_ref::HttpHostRef::from(
                    constants_str::catalog::PG_CRUD_EMPTY_SQL_SUFFIX,
                )
            });
            let span = tracing::info_span!(
                "http.client",
                otel.kind = "client",
                otel.name = tracing::field::Empty,
                otel.status_code = tracing::field::Empty,
                "http.request.method" = %method,
                "server.address" = %host,
                "http.response.status_code" = tracing::field::Empty,
            );
            let _client_name_record = span.record(
                constants_str::test_fixtures::OTEL_NAME,
                format_args!("{method} {host}"),
            );
            span
        };
        crate::inject_trace_context::inject_trace_context(
            &crate::opentelemetry_context::OpentelemetryContext::from(
                tracing_opentelemetry::OpenTelemetrySpanExt::context(&span),
            ),
            request.headers_mut(),
        );
        super::tracing_http_client_span::TracingHttpClientSpan::from(span)
    }

    pub fn try_new(
        policy: super::reqwest_client_policy::ReqwestClientPolicy,
    ) -> Result<Self, super::reqwest_client_build_error::ReqwestClientBuildError> {
        reqwest::Client::builder()
            .connect_timeout(*policy.connect_timeout())
            .timeout(*policy.request_timeout())
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .map(Self)
            .map_err(super::reqwest_client_build_error::ReqwestClientBuildError::from)
    }
}
