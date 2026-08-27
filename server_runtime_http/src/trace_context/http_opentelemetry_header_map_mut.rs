#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    newtype::DerefInner,
    newtype::DerefMutInner,
    newtype::FromInner,
)]
pub struct HttpOpentelemetryHeaderMapMut<'headers_lt>(pub(super) &'headers_lt mut http::HeaderMap);
