#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct HttpNotificationHeaderMap(pub(super) http::HeaderMap);
