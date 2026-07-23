const TRACE_PARENT_LEN: usize = 55;
const TRACE_STATE_MAX_LEN: usize = 512;

#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct HttpTraceParent(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
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

#[derive(Clone, Debug, Eq, PartialEq, newtype::AsRefStr)]
pub struct HttpTraceState(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutboundTraceContext {
    request_id: Option<crate::RequestId>,
    trace_parent: HttpTraceParent,
    trace_state: Option<HttpTraceState>,
}

#[derive(Debug, newtype::FromInner, newtype::IntoInnerFrom)]
pub struct ReqwestRequestBuilder(reqwest::RequestBuilder);

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

#[cfg(test)]
mod tests {
    #[test]
    fn validates_and_applies_w3c_trace_context() {
        let trace_parent =
            super::HttpTraceParent::try_from(str_constants::TRACEPARENT_TEST_VALUE.to_owned())
                .expect("6b490bf8");
        let trace_state =
            super::HttpTraceState::try_from(str_constants::TRACESTATE_TEST_VALUE.to_owned())
                .expect("b82fb9ef");
        let request_id =
            crate::RequestId::try_from(str_constants::REQUEST_ID_TEST_VALUE.to_owned())
                .expect("50c01ea8");
        let client = crate::ReqwestClient::try_new(crate::ReqwestClientPolicy::new(
            crate::StdReqwestConnectTimeout::try_from(std::time::Duration::from_secs(1u64))
                .expect("ce032a9f"),
            crate::StdReqwestRequestTimeout::try_from(std::time::Duration::from_secs(2u64))
                .expect("a1dabed3"),
        ))
        .expect("8ded9d63");
        let request_builder: reqwest::RequestBuilder =
            super::OutboundTraceContext::new(trace_parent, Some(trace_state), Some(request_id))
                .apply(
                    reqwest::Client::from(client)
                        .get(str_constants::HTTPS_EXAMPLE_COM)
                        .into(),
                )
                .into();
        let request = request_builder.build().expect("1574578f");
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
}
