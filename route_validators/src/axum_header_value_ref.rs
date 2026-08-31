#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    newtype::DerefTarget,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct AxumHeaderValueRef<'header_value_lt>(&'header_value_lt axum::http::HeaderValue);
