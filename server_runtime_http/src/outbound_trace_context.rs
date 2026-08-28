#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct OutboundTraceContext {
    request_id: Option<crate::domain_types::RequestId>,
    trace_parent: super::HttpTraceParent,
    trace_state: Option<super::HttpTraceState>,
}

impl OutboundTraceContext {
    #[must_use]
    pub fn apply(&self, request: super::ReqwestRequestBuilder) -> super::ReqwestRequestBuilder {
        let request_with_parent = request
            .0
            .header(constants_str::TRACEPARENT, self.trace_parent.as_ref());
        let request_with_state = match self.trace_state.as_ref() {
            Some(trace_state) => {
                request_with_parent.header(constants_str::TRACESTATE, trace_state.as_ref())
            }
            None => request_with_parent,
        };
        match self.request_id.as_ref() {
            Some(request_id) => {
                request_with_state.header(constants_str::X_REQUEST_ID, request_id.to_string())
            }
            None => request_with_state,
        }
        .into()
    }

    #[must_use]
    pub const fn new(
        trace_parent: super::HttpTraceParent,
        trace_state: Option<super::HttpTraceState>,
        request_id: Option<crate::domain_types::RequestId>,
    ) -> Self {
        Self {
            request_id,
            trace_parent,
            trace_state,
        }
    }
}
