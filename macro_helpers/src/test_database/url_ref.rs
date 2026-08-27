#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub struct UrlRef<'url_lt>(pub(super) &'url_lt str);
