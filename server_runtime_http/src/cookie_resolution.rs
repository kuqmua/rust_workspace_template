#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq,
)]
pub enum CookieResolution<'value_lt> {
    Invalid,
    Missing,
    Resolved(crate::http_cookie_value_ref::HttpCookieValueRef<'value_lt>),
}
