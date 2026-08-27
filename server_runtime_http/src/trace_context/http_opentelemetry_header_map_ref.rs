#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    newtype::DerefInner,
    newtype::FromInner,
)]
pub struct HttpOpentelemetryHeaderMapRef<'headers_lt>(pub(super) &'headers_lt http::HeaderMap);
