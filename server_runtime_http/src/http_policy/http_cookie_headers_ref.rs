#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpCookieHeadersRef<'value_lt>(pub(super) &'value_lt http::HeaderMap);
