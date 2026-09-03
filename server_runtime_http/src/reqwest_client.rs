#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    proc_macro_newtype_from_inner::FromInner,
    proc_macro_newtype_into_inner_from::IntoInnerFrom,
)]
pub struct ReqwestClient(reqwest::Client);
impl ReqwestClient {
    pub async fn execute(
        &self,
        mut reqwest_request: crate::reqwest_request::ReqwestRequest,
    ) -> Result<crate::reqwest_response::ReqwestResponse, crate::reqwest_error::ReqwestError> {
        let span = Self::prepare_observed_http_request(&mut reqwest_request);
        tracing::Instrument::instrument(
            async {
                match self.0.execute(reqwest_request.into_inner()).await {
                    Ok(response) => {
                        let _client_status_record = tracing::Span::current().record(
                            constants_str::OTEL_HTTP_RESPONSE_STATUS_CODE,
                            response.status().as_u16(),
                        );
                        if response.status().is_server_error() {
                            let _client_error_record = tracing::Span::current().record(
                                constants_str::OTEL_STATUS_CODE,
                                constants_str::OTEL_ERROR_STATUS,
                            );
                        }
                        Ok(crate::reqwest_response::ReqwestResponse::from(response))
                    }
                    Err(error) => {
                        let _client_error_record = tracing::Span::current().record(
                            constants_str::OTEL_STATUS_CODE,
                            constants_str::OTEL_ERROR_STATUS,
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
        reason = "request instrumentation preparation is shared with deterministic test_tests"
    )]
    pub(crate) fn prepare_observed_http_request(
        reqwest_request: &mut crate::reqwest_request::ReqwestRequest,
    ) -> super::tracing_http_client_span::TracingHttpClientSpan {
        let span = {
            let method = reqwest_request.method();
            let host = reqwest_request.host().unwrap_or_else(|| {
                crate::http_host_ref::HttpHostRef::from(constants_str::PG_CRUD_EMPTY_SQL_SUFFIX)
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
            let _client_name_record =
                span.record(constants_str::OTEL_NAME, format_args!("{method} {host}"));
            span
        };
        crate::inject_trace_context::inject_trace_context(
            &crate::opentelemetry_context::OpentelemetryContext::from(
                tracing_opentelemetry::OpenTelemetrySpanExt::context(&span),
            ),
            reqwest_request.headers_mut(),
        );
        super::tracing_http_client_span::TracingHttpClientSpan::from(span)
    }

    pub fn try_new(
        reqwest_client_policy: super::reqwest_client_policy::ReqwestClientPolicy,
    ) -> Result<Self, super::reqwest_client_build_error::ReqwestClientBuildError> {
        reqwest::Client::builder()
            .connect_timeout(*reqwest_client_policy.connect_timeout())
            .timeout(*reqwest_client_policy.request_timeout())
            .dns_resolver(crate::outbound_dns_resolver::OutboundDnsResolver::new(
                reqwest_client_policy.host_policy(),
            ))
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
