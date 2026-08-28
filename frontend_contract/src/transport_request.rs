#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug, PartialEq, Eq)]
pub struct TransportRequest {
    body: super::TransportBody,
    path: super::TransportPath,
    route: crate::RouteContract,
    idempotency_key: Option<super::TransportIdempotencyKey>,
    if_match: Option<super::TransportIfMatch>,
}

impl TransportRequest {
    #[must_use]
    pub const fn new(
        body: super::TransportBody,
        path: super::TransportPath,
        route: crate::RouteContract,
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
    pub const fn body(&self) -> &super::TransportBody {
        &self.body
    }
    #[must_use]
    pub const fn path(&self) -> &super::TransportPath {
        &self.path
    }
    #[must_use]
    pub const fn route(&self) -> crate::RouteContract {
        self.route
    }
    #[must_use]
    pub const fn idempotency_key(&self) -> Option<&super::TransportIdempotencyKey> {
        self.idempotency_key.as_ref()
    }
    #[must_use]
    pub const fn if_match(&self) -> Option<&super::TransportIfMatch> {
        self.if_match.as_ref()
    }
    #[must_use]
    pub fn with_idempotency_key(mut self, value: super::TransportIdempotencyKey) -> Self {
        self.idempotency_key = Some(value);
        self
    }
    #[must_use]
    pub fn with_if_match(mut self, value: super::TransportIfMatch) -> Self {
        self.if_match = Some(value);
        self
    }
}
