#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, newtype::FromInner)]
pub(super) struct CommandSucceeded(pub(super) bool);
impl CommandSucceeded {
    pub(super) const fn get(self) -> bool {
        self.0
    }
}
