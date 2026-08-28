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
    pub fn as_str(self) -> crate::ContractStr {
        crate::ContractStr::from(match self {
            Self::Connect => constants_str::CONNECT,
            Self::Delete => constants_str::DELETE,
            Self::Get => constants_str::GET,
            Self::Head => constants_str::HEAD,
            Self::Options => constants_str::OPTIONS,
            Self::Patch => constants_str::PATCH,
            Self::Post => constants_str::POST,
            Self::Put => constants_str::PUT,
            Self::Trace => constants_str::TRACE,
        })
    }
}
