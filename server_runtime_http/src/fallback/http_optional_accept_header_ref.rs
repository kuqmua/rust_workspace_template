#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpOptionalAcceptHeaderRef<'value_lt>(pub(super) Option<&'value_lt http::HeaderValue>);
