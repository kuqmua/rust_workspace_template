#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpHeaderMapRef<'lt>(pub(super) &'lt http::HeaderMap);
