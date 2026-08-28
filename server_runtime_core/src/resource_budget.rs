#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct ResourceBudget {
    maximum: ResourceBudgetMaximum,
    reserved: SharedAtomicUsizeArc,
}

impl ResourceBudget {
    #[must_use]
    pub fn new(maximum: ResourceBudgetMaximum) -> Self {
        Self {
            maximum,
            reserved: SharedAtomicUsizeArc::from(std::sync::Arc::from(
                std::sync::atomic::AtomicUsize::new(constants_usize::ZERO),
            )),
        }
    }

    pub fn reserve(
        &self,
        amount: ResourceBudgetAmount,
    ) -> Result<ResourceBudgetReservation, ResourceBudgetReserveError> {
        let result = self.reserved.0.try_update(
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
            |current| {
                current
                    .checked_add(amount.0)
                    .filter(|next| *next <= self.maximum.0.0.get())
            },
        );
        match result {
            Ok(_previous) => Ok(ResourceBudgetReservation {
                amount,
                reserved: self.reserved.clone(),
            }),
            Err(current) if current.checked_add(amount.0).is_none() => {
                Err(ResourceBudgetReserveError::Overflow)
            }
            Err(_current) => Err(ResourceBudgetReserveError::Exhausted),
        }
    }

    #[must_use]
    pub fn reserved(&self) -> ResourceBudgetAmount {
        ResourceBudgetAmount::from(self.reserved.0.load(std::sync::atomic::Ordering::Acquire))
    }
}

pub use crate::bulk_item_resource_budget_provider::BulkItemResourceBudgetProvider;
pub use crate::idempotency_response_resource_budget_provider::IdempotencyResponseResourceBudgetProvider;
pub use crate::resource_budget_amount::ResourceBudgetAmount;
pub use crate::resource_budget_config_error::ResourceBudgetConfigError;
pub use crate::resource_budget_maximum::ResourceBudgetMaximum;
use crate::resource_budget_maximum_non_zero_usize::ResourceBudgetMaximumNonZeroUsize;
pub use crate::resource_budget_reservation::ResourceBudgetReservation;
pub use crate::resource_budget_reserve_error::ResourceBudgetReserveError;
use crate::shared_atomic_usize_arc::SharedAtomicUsizeArc;
