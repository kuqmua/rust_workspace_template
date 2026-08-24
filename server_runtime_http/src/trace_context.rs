const TRACE_PARENT_LEN: usize = 55;
const TRACE_STATE_MAX_LEN: usize = 512;

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, newtype::FromInner)]
struct HttpHeaderExtractor<'headers_lt>(&'headers_lt http::HeaderMap);

impl opentelemetry::propagation::Extractor for HttpHeaderExtractor<'_> {
    fn get(&self, key: &str) -> Option<&str> {
        let value = self.0.get(key)?;
        value.to_str().ok()
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(http::HeaderName::as_str).collect()
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
struct HttpHeaderInjector<'headers_lt>(&'headers_lt mut http::HeaderMap);

impl opentelemetry::propagation::Injector for HttpHeaderInjector<'_> {
    fn set(&mut self, key: &str, value: String) {
        let Ok(header_name) = http::HeaderName::try_from(key) else {
            return;
        };
        let Ok(header_value) = http::HeaderValue::try_from(value) else {
            return;
        };
        let _previous_value = self.0.insert(header_name, header_value);
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct HttpTraceParent(String);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
pub enum HttpTraceParentError {
    #[error("{}", str_constants::TRACEPARENT_W3C_VERSION_00_FORMAT)]
    Format,
    #[error("{}", str_constants::TRACEPARENT_PARENT_ID_NOT_ZERO)]
    ZeroParentId,
    #[error("{}", str_constants::TRACEPARENT_TRACE_ID_NOT_ZERO)]
    ZeroTraceId,
}

impl TryFrom<String> for HttpTraceParent {
    type Error = HttpTraceParentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        if bytes.len() != TRACE_PARENT_LEN
            || bytes.get(0usize..3usize) != Some(b"00-")
            || bytes.get(35usize) != Some(&b'-')
            || bytes.get(52usize) != Some(&b'-')
            || !bytes
                .iter()
                .enumerate()
                .filter(|(idx, _byte)| !matches!(idx, 2usize | 35usize | 52usize))
                .all(|(_idx, byte)| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(Self::Error::Format);
        }
        let Some(trace_id) = bytes.get(3usize..35usize) else {
            return Err(Self::Error::Format);
        };
        let Some(parent_id) = bytes.get(36usize..52usize) else {
            return Err(Self::Error::Format);
        };
        if trace_id.iter().all(|byte| *byte == b'0') {
            return Err(Self::Error::ZeroTraceId);
        }
        if parent_id.iter().all(|byte| *byte == b'0') {
            return Err(Self::Error::ZeroParentId);
        }
        Ok(Self(value))
    }
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq, newtype::AsRefStr,
)]
pub struct HttpTraceState(String);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq, thiserror::Error,
)]
#[error("{}", str_constants::TRACESTATE_PRINTABLE_ASCII_MAX_512)]
pub struct HttpTraceStateError;

impl TryFrom<String> for HttpTraceState {
    type Error = HttpTraceStateError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.len() > TRACE_STATE_MAX_LEN
            || !value.bytes().all(|byte| (0x20u8..=0x7eu8).contains(&byte))
        {
            return Err(HttpTraceStateError);
        }
        Ok(Self(value))
    }
}

#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct OutboundTraceContext {
    request_id: Option<crate::RequestId>,
    trace_parent: HttpTraceParent,
    trace_state: Option<HttpTraceState>,
}

#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::IntoInnerFrom,
)]
pub struct ReqwestRequestBuilder(reqwest::RequestBuilder);

#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner)]
pub struct ReqwestRequest(reqwest::Request);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub struct HttpOpentelemetryHeaderMapMut<'headers_lt>(&'headers_lt mut http::HeaderMap);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct HttpOpentelemetryHeaderMapRef<'headers_lt>(&'headers_lt http::HeaderMap);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::Display,
    newtype::FromInner,
)]
pub struct HttpHostRef<'host_lt>(&'host_lt str);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::Display,
    newtype::FromInner,
)]
pub struct HttpMethodRef<'method_lt>(&'method_lt http::Method);

#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct OpentelemetryContext(opentelemetry::Context);

impl ReqwestRequest {
    pub(crate) fn headers_mut(&mut self) -> HttpOpentelemetryHeaderMapMut<'_> {
        HttpOpentelemetryHeaderMapMut::from(self.0.headers_mut())
    }

    pub(crate) fn host(&self) -> Option<HttpHostRef<'_>> {
        self.0.url().host_str().map(HttpHostRef::from)
    }

    pub(crate) fn into_inner(self) -> reqwest::Request {
        self.0
    }

    pub(crate) fn method(&self) -> HttpMethodRef<'_> {
        HttpMethodRef::from(self.0.method())
    }
}

impl TryFrom<ReqwestRequestBuilder> for ReqwestRequest {
    type Error = crate::ReqwestError;

    fn try_from(value: ReqwestRequestBuilder) -> Result<Self, Self::Error> {
        value.0.build().map(Self).map_err(crate::ReqwestError::from)
    }
}

impl OutboundTraceContext {
    #[must_use]
    pub fn apply(&self, request: ReqwestRequestBuilder) -> ReqwestRequestBuilder {
        let request_with_parent = request
            .0
            .header(str_constants::TRACEPARENT, self.trace_parent.as_ref());
        let request_with_state = match self.trace_state.as_ref() {
            Some(trace_state) => {
                request_with_parent.header(str_constants::TRACESTATE, trace_state.as_ref())
            }
            None => request_with_parent,
        };
        match self.request_id.as_ref() {
            Some(request_id) => {
                request_with_state.header(str_constants::X_REQUEST_ID, request_id.to_string())
            }
            None => request_with_state,
        }
        .into()
    }

    #[must_use]
    pub const fn new(
        trace_parent: HttpTraceParent,
        trace_state: Option<HttpTraceState>,
        request_id: Option<crate::RequestId>,
    ) -> Self {
        Self {
            request_id,
            trace_parent,
            trace_state,
        }
    }
}

#[must_use]
pub fn extract_remote_trace_context(
    headers: HttpOpentelemetryHeaderMapRef<'_>,
) -> OpentelemetryContext {
    opentelemetry::global::get_text_map_propagator(|propagator| {
        OpentelemetryContext::from(propagator.extract(&HttpHeaderExtractor::from(headers.0)))
    })
}

pub fn inject_trace_context(
    context: &OpentelemetryContext,
    mut headers: HttpOpentelemetryHeaderMapMut<'_>,
) {
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&context.0, &mut HttpHeaderInjector::from(&mut **headers));
    });
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg_attr(
        miri,
        ignore = "native TLS initialization calls OpenSSL functions that Miri does not support"
    )]
    fn validates_and_applies_w3c_trace_context() {
        let trace_parent =
            super::HttpTraceParent::try_from(str_constants::TRACEPARENT_TEST_VALUE.to_owned())
                .expect("6b490bf8 validates_and_applies_w3c_trace_context invariant must hold");
        let trace_state =
            super::HttpTraceState::try_from(str_constants::TRACESTATE_TEST_VALUE.to_owned())
                .expect("b82fb9ef validates_and_applies_w3c_trace_context invariant must hold");
        let request_id =
            crate::RequestId::try_from(str_constants::REQUEST_ID_TEST_VALUE.to_owned())
                .expect("50c01ea8 validates_and_applies_w3c_trace_context invariant must hold");
        let client = crate::ReqwestClient::try_new(crate::ReqwestClientPolicy::new(
            crate::StdReqwestConnectTimeout::try_from(std::time::Duration::from_secs(1u64))
                .expect("ce032a9f validates_and_applies_w3c_trace_context invariant must hold"),
            crate::StdReqwestRequestTimeout::try_from(std::time::Duration::from_secs(2u64))
                .expect("a1dabed3 validates_and_applies_w3c_trace_context invariant must hold"),
        ))
        .expect("8ded9d63 validates_and_applies_w3c_trace_context invariant must hold");
        let request_builder: reqwest::RequestBuilder =
            super::OutboundTraceContext::new(trace_parent, Some(trace_state), Some(request_id))
                .apply(
                    reqwest::Client::from(client)
                        .get(str_constants::HTTPS_EXAMPLE_COM)
                        .into(),
                )
                .into();
        let request = request_builder
            .build()
            .expect("1574578f validates_and_applies_w3c_trace_context invariant must hold");
        assert_eq!(
            request.headers()[str_constants::TRACESTATE],
            str_constants::TRACESTATE_TEST_VALUE
        );
        assert_eq!(
            request.headers()[str_constants::X_REQUEST_ID],
            str_constants::REQUEST_ID_TEST_VALUE
        );
    }

    #[test]
    fn rejects_zero_identifiers() {
        assert_eq!(
            super::HttpTraceParent::try_from(
                str_constants::TRACEPARENT_ZERO_TRACE_ID_TEST_VALUE.to_owned(),
            ),
            Err(super::HttpTraceParentError::ZeroTraceId)
        );
    }

    #[test]
    fn extracts_valid_w3c_parent_context() {
        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );
        let mut headers = http::HeaderMap::new();
        let _previous = headers.insert(
            http::HeaderName::from_static(str_constants::TRACEPARENT),
            http::HeaderValue::from_static(str_constants::TRACEPARENT_TEST_VALUE),
        );
        let context = super::extract_remote_trace_context(
            super::HttpOpentelemetryHeaderMapRef::from(&headers),
        );
        let span = opentelemetry::trace::TraceContextExt::span(&context.0);
        assert!(span.span_context().is_remote());
        let expected_trace_id = str_constants::TRACEPARENT_TEST_VALUE
            .get(3usize..35usize)
            .expect("65aa5eca extracts_valid_w3c_parent_context invariant must hold");
        assert_eq!(
            span.span_context().trace_id().to_string(),
            expected_trace_id
        );
    }

    #[test]
    fn injects_w3c_context() {
        opentelemetry::global::set_text_map_propagator(
            opentelemetry_sdk::propagation::TraceContextPropagator::new(),
        );
        let headers = http::HeaderMap::from_iter([
            (
                http::HeaderName::from_static(str_constants::TRACEPARENT),
                http::HeaderValue::from_static(str_constants::TRACEPARENT_TEST_VALUE),
            ),
            (
                http::HeaderName::from_static(str_constants::TRACESTATE),
                http::HeaderValue::from_static(str_constants::TRACESTATE_TEST_VALUE),
            ),
        ]);
        let context = super::extract_remote_trace_context(
            super::HttpOpentelemetryHeaderMapRef::from(&headers),
        );
        let mut injected_headers = http::HeaderMap::new();
        super::inject_trace_context(
            &context,
            super::HttpOpentelemetryHeaderMapMut::from(&mut injected_headers),
        );
        assert_eq!(
            injected_headers.get(str_constants::TRACEPARENT),
            Some(&http::HeaderValue::from_static(
                str_constants::TRACEPARENT_TEST_VALUE
            ))
        );
        assert_eq!(
            injected_headers.get(str_constants::TRACESTATE),
            Some(&http::HeaderValue::from_static(
                str_constants::TRACESTATE_TEST_VALUE
            ))
        );
    }
}
