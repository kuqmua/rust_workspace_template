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

#[cfg(test)]
mod tests {
    #[test]
    fn test_idempotency_conversions_preserve_independent_boolean_and_variant_meanings() {
        assert!(matches!(
            super::IdempotencyCapability::from(false),
            super::IdempotencyCapability::Disabled
        ));
        assert!(matches!(
            super::IdempotencyCapability::from(true),
            super::IdempotencyCapability::Enabled
        ));
        assert!(!bool::from(super::IdempotencyCapability::Disabled));
        assert!(bool::from(super::IdempotencyCapability::Enabled));
    }
}
