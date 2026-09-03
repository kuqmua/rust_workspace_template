#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Copy)]
pub(crate) enum OptimisticConcurrencyCapability {
    Disabled,
    Enabled,
}
impl From<bool> for OptimisticConcurrencyCapability {
    fn from(value: bool) -> Self {
        if value { Self::Enabled } else { Self::Disabled }
    }
}
impl From<OptimisticConcurrencyCapability> for bool {
    fn from(value: OptimisticConcurrencyCapability) -> Self {
        matches!(value, OptimisticConcurrencyCapability::Enabled)
    }
}
