#[derive(optimal_memory_layout::OptimalMemoryLayout, newtype::FromInner)]
pub(super) struct HttpNotificationHeaderMap(http::HeaderMap);

impl HttpNotificationHeaderMap {
    pub(crate) const fn get(&self) -> &http::HeaderMap {
        &self.0
    }
}
