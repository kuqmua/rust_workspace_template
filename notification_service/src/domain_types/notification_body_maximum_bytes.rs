#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(crate) struct NotificationBodyMaximumBytes(usize);
impl NotificationBodyMaximumBytes {
    pub(crate) const fn get(self) -> usize {
        self.0
    }
}
