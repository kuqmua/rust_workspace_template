#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResourceBudgetMaximum(pub(super) super::ResourceBudgetMaximumNonZeroUsize);

impl TryFrom<usize> for ResourceBudgetMaximum {
    type Error = super::ResourceBudgetConfigError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self::from)
            .ok_or(super::ResourceBudgetConfigError)
    }
}

impl From<std::num::NonZeroUsize> for ResourceBudgetMaximum {
    fn from(value: std::num::NonZeroUsize) -> Self {
        Self(super::ResourceBudgetMaximumNonZeroUsize::from(value))
    }
}
