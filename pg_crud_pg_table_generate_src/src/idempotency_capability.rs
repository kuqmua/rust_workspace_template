#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) enum IdempotencyCapability {
    Disabled,
    Enabled,
}
impl From<bool> for IdempotencyCapability {
    fn from(value: bool) -> Self {
        if value { Self::Enabled } else { Self::Disabled }
    }
}
impl From<IdempotencyCapability> for bool {
    fn from(value: IdempotencyCapability) -> Self {
        matches!(value, IdempotencyCapability::Enabled)
    }
}
