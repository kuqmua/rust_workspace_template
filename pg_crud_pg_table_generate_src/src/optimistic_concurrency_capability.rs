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

#[cfg(test)]
mod tests {
    #[test]
    fn test_optimistic_concurrency_conversions_preserve_independent_boolean_and_variant_meanings() {
        assert!(matches!(
            super::OptimisticConcurrencyCapability::from(false),
            super::OptimisticConcurrencyCapability::Disabled
        ));
        assert!(matches!(
            super::OptimisticConcurrencyCapability::from(true),
            super::OptimisticConcurrencyCapability::Enabled
        ));
        assert!(!bool::from(
            super::OptimisticConcurrencyCapability::Disabled
        ));
        assert!(bool::from(super::OptimisticConcurrencyCapability::Enabled));
    }
}
