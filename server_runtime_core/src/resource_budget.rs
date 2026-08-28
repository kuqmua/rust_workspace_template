#![allow(
    clippy::arbitrary_source_item_ordering,
    clippy::module_inception,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[path = "bulk_item_resource_budget_provider.rs"]
mod bulk_item_resource_budget_provider;
#[path = "idempotency_response_resource_budget_provider.rs"]
mod idempotency_response_resource_budget_provider;
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
#[path = "resource_budget_amount.rs"]
mod resource_budget_amount;
#[path = "resource_budget_config_error.rs"]
mod resource_budget_config_error;
#[path = "resource_budget_maximum.rs"]
mod resource_budget_maximum;
#[path = "resource_budget_maximum_non_zero_usize.rs"]
mod resource_budget_maximum_non_zero_usize;
#[path = "resource_budget_reservation.rs"]
mod resource_budget_reservation;
#[path = "resource_budget_reserve_error.rs"]
mod resource_budget_reserve_error;
#[path = "shared_atomic_usize_arc.rs"]
mod shared_atomic_usize_arc;

pub use bulk_item_resource_budget_provider::BulkItemResourceBudgetProvider;
pub use idempotency_response_resource_budget_provider::IdempotencyResponseResourceBudgetProvider;
pub use resource_budget_amount::ResourceBudgetAmount;
pub use resource_budget_config_error::ResourceBudgetConfigError;
pub use resource_budget_maximum::ResourceBudgetMaximum;
use resource_budget_maximum_non_zero_usize::ResourceBudgetMaximumNonZeroUsize;
pub use resource_budget_reservation::ResourceBudgetReservation;
pub use resource_budget_reserve_error::ResourceBudgetReserveError;
use shared_atomic_usize_arc::SharedAtomicUsizeArc;
