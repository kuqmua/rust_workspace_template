#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ServerApiBoundary(route_validators::RouteValidationStatusCode);

impl ServerApiBoundary {
    #[must_use]
    pub const fn new(status_code: route_validators::RouteValidationStatusCode) -> Self {
        Self(status_code)
    }
}
