#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub enum CookieResolution<'value_lt> {
    Invalid,
    Missing,
    Resolved(super::HttpCookieValueRef<'value_lt>),
}
