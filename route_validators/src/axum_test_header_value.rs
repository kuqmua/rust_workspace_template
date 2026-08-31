#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub(crate) struct AxumTestHeaderValue(axum::http::HeaderValue);
