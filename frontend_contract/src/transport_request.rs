#[derive(proc_macro_getters::Getters)]
#[getters(bare)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct TransportRequest {
    body: crate::transport_body::TransportBody,
    path: crate::transport_path::TransportPath,
    #[getters(copy)]
    route: crate::route_contract::RouteContract,
    #[getters(skip)]
    idempotency_key: Option<crate::transport_idempotency_key::TransportIdempotencyKey>,
    #[getters(skip)]
    if_match: Option<crate::transport_if_match::TransportIfMatch>,
}

impl TransportRequest {
    #[must_use]
    pub const fn new(
        body: crate::transport_body::TransportBody,
        path: crate::transport_path::TransportPath,
        route: crate::route_contract::RouteContract,
    ) -> Self {
        Self {
            body,
            path,
            route,
            idempotency_key: None,
            if_match: None,
        }
    }

    #[must_use]
    pub const fn idempotency_key(
        &self,
    ) -> Option<&crate::transport_idempotency_key::TransportIdempotencyKey> {
        self.idempotency_key.as_ref()
    }
    #[must_use]
    pub const fn if_match(&self) -> Option<&crate::transport_if_match::TransportIfMatch> {
        self.if_match.as_ref()
    }
    #[must_use]
    pub fn with_idempotency_key(
        mut self,
        value: crate::transport_idempotency_key::TransportIdempotencyKey,
    ) -> Self {
        self.idempotency_key = Some(value);
        self
    }
    #[must_use]
    pub fn with_if_match(mut self, value: crate::transport_if_match::TransportIfMatch) -> Self {
        self.if_match = Some(value);
        self
    }
}
