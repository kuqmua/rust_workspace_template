#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpAuthorizationHeaderTextRef<'value_lt>(pub(super) Option<&'value_lt str>);
