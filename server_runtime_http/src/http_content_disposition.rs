#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Debug,
    newtype::FromInner,
    newtype::IntoInnerFrom,
)]
pub struct HttpContentDisposition(http::HeaderValue);
