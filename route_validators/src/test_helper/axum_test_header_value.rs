#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::DerefInner, newtype::FromInner)]
pub(crate) struct AxumTestHeaderValue(pub(super) axum::http::HeaderValue);
