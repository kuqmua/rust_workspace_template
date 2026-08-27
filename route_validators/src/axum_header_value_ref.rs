#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::FromInner,
)]
pub(crate) struct AxumHeaderValueRef<'header_value_lt>(
    pub(super) &'header_value_lt axum::http::HeaderValue,
);
