#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::AsRefOwned,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub(crate) struct AxumTestHeaders(axum::http::HeaderMap);
