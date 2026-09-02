#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum BearerAuthorizationResolution<'value_lt> {
    Invalid,
    Missing,
    Resolved(crate::http_bearer_token_ref::HttpBearerTokenRef<'value_lt>),
}
