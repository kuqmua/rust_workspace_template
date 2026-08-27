#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum BearerAuthorizationResolution<'value_lt> {
    Invalid,
    Missing,
    Resolved(super::HttpBearerTokenRef<'value_lt>),
}
