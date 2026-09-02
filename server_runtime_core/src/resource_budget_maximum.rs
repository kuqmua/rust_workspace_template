#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    proc_macro_newtype::DerefInner,
    proc_macro_newtype::FromInner,
)]
pub struct ResourceBudgetMaximum(std::num::NonZeroUsize);

impl TryFrom<usize> for ResourceBudgetMaximum {
    type Error = crate::resource_budget_config_error::ResourceBudgetConfigError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        std::num::NonZeroUsize::new(value)
            .map(Self::from)
            .ok_or(crate::resource_budget_config_error::ResourceBudgetConfigError::Zero)
    }
}
