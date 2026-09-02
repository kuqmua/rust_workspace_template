#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, Eq, PartialEq)]
pub struct OutboundTraceContext {
    request_id: Option<crate::request_id::RequestId>,
    trace_parent: crate::http_trace_parent::HttpTraceParent,
    trace_state: Option<crate::http_trace_state::HttpTraceState>,
}

impl OutboundTraceContext {
    #[must_use]
    pub fn apply(
        &self,
        request: crate::reqwest_request_builder::ReqwestRequestBuilder,
    ) -> crate::reqwest_request_builder::ReqwestRequestBuilder {
        let request_with_parent = reqwest::RequestBuilder::from(request)
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
        trace_parent: crate::http_trace_parent::HttpTraceParent,
        trace_state: Option<crate::http_trace_state::HttpTraceState>,
        request_id: Option<crate::request_id::RequestId>,
    ) -> Self {
        Self {
            request_id,
            trace_parent,
            trace_state,
        }
    }
}
