#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct HttpOriginHeadersRef<'header>(pub(super) &'header http::HeaderMap);
