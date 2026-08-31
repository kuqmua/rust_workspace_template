#[derive(
    Debug,
    Clone,
    Copy,
    optimal_memory_layout::OptimalMemoryLayout,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub(crate) struct AxumHttpUriRef<'uri_lt>(&'uri_lt axum::http::Uri);
