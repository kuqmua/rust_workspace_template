#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(
    optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    newtype::FromInner,
)]
pub struct ResourceBudgetMaximum(pub(super) std::num::NonZeroUsize);

impl TryFrom<usize> for ResourceBudgetMaximum {
    type Error = crate::resource_budget_config_error::ResourceBudgetConfigError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self::from)
            .ok_or(crate::resource_budget_config_error::ResourceBudgetConfigError)
    }
}
