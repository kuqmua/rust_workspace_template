#[derive(optml::Optml, Clone, Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct ReqwestClient(reqwest::Client);

#[derive(optml::Optml, Clone, Copy, Debug)]
pub struct StdReqwestConnectTimeout(std::time::Duration);

#[derive(optml::Optml, Clone, Copy, Debug)]
pub struct StdReqwestRequestTimeout(std::time::Duration);

#[derive(optml::Optml, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("HTTP client timeout must be greater than zero")]
pub struct StdReqwestTimeoutError;

impl TryFrom<std::time::Duration> for StdReqwestConnectTimeout {
    type Error = StdReqwestTimeoutError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdReqwestTimeoutError)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<std::time::Duration> for StdReqwestRequestTimeout {
    type Error = StdReqwestTimeoutError;

    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        if value.is_zero() {
            Err(StdReqwestTimeoutError)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(optml::Optml, Clone, Copy, Debug)]
pub struct ReqwestClientPolicy {
    connect_timeout: StdReqwestConnectTimeout,
    request_timeout: StdReqwestRequestTimeout,
}

impl ReqwestClientPolicy {
    #[must_use]
    pub const fn new(
        connect_timeout: StdReqwestConnectTimeout,
        request_timeout: StdReqwestRequestTimeout,
    ) -> Self {
        Self {
            connect_timeout,
            request_timeout,
        }
    }
}

#[derive(optml::Optml, Debug, thiserror::Error, newtype::FromInner)]
#[error(transparent)]
pub struct ReqwestClientBuildError(reqwest::Error);

#[derive(optml::Optml, Debug, newtype::FromInner)]
pub(super) struct TracingHttpClientSpan(tracing::Span);

impl TracingHttpClientSpan {
    fn into_inner(self) -> tracing::Span {
        self.0
    }
}

impl ReqwestClient {
    pub async fn execute(
        &self,
        mut request: super::ReqwestRequest,
    ) -> Result<super::ReqwestResponse, super::ReqwestError> {
        let span = Self::prepare_observed_http_request(&mut request);
        tracing::Instrument::instrument(
            async {
                match self.0.execute(request.into_inner()).await {
                    Ok(response) => {
                        let _client_status_record = tracing::Span::current().record(
                            str_constants::OTEL_HTTP_RESPONSE_STATUS_CODE,
                            response.status().as_u16(),
                        );
                        if response.status().is_server_error() {
                            let _client_error_record = tracing::Span::current().record(
                                str_constants::OTEL_STATUS_CODE,
                                str_constants::OTEL_ERROR_STATUS,
                            );
                        }
                        Ok(super::ReqwestResponse::from(response))
                    }
                    Err(error) => {
                        let _client_error_record = tracing::Span::current().record(
                            str_constants::OTEL_STATUS_CODE,
                            str_constants::OTEL_ERROR_STATUS,
                        );
                        Err(super::ReqwestError::from(error))
                    }
                }
            },
            span.into_inner(),
        )
        .await
    }

    #[allow(clippy::single_call_fn)] // shared preparation keeps production execution and deterministic propagation tests on the same implementation
    pub(super) fn prepare_observed_http_request(
        request: &mut super::ReqwestRequest,
    ) -> TracingHttpClientSpan {
        let span = {
            let method = request.method();
            let host = request
                .host()
                .unwrap_or_else(|| super::HttpHostRef::from(""));
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
                span.record(str_constants::OTEL_NAME, format_args!("{method} {host}"));
            span
        };
        super::inject_trace_context(
            &super::OpentelemetryContext::from(
                tracing_opentelemetry::OpenTelemetrySpanExt::context(&span),
            ),
            request.headers_mut(),
        );
        TracingHttpClientSpan::from(span)
    }

    pub fn try_new(policy: ReqwestClientPolicy) -> Result<Self, ReqwestClientBuildError> {
        reqwest::Client::builder()
            .connect_timeout(policy.connect_timeout.0)
            .timeout(policy.request_timeout.0)
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .map(Self)
            .map_err(ReqwestClientBuildError)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn timeout_wrappers_reject_zero() {
        assert_eq!(
            super::StdReqwestConnectTimeout::try_from(std::time::Duration::ZERO).err(),
            Some(super::StdReqwestTimeoutError)
        );
        assert_eq!(
            super::StdReqwestRequestTimeout::try_from(std::time::Duration::ZERO).err(),
            Some(super::StdReqwestTimeoutError)
        );
    }
}
