#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteMethod {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
}
impl RouteMethod {
    #[must_use]
    pub fn as_str(self) -> crate::contract_str::ContractStr {
        crate::contract_str::ContractStr::from(match self {
            Self::Connect => constants_str::catalog::CONNECT,
            Self::Delete => constants_str::integration_fixtures::DELETE,
            Self::Get => constants_str::catalog::GET,
            Self::Head => constants_str::test_fixtures::HEAD,
            Self::Options => constants_str::test_fixtures::OPTIONS,
            Self::Patch => constants_str::catalog::PATCH,
            Self::Post => constants_str::catalog::POST,
            Self::Put => constants_str::catalog::PUT,
            Self::Trace => constants_str::test_fixtures::TRACE,
        })
    }
}
