#![allow(
    clippy::arbitrary_source_item_ordering,
    reason = "the flat source facade keeps its owner adjacent to implementation while declaring sibling modules"
)]
#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct ResourceBudget {
    maximum: crate::resource_budget_maximum::ResourceBudgetMaximum,
    reserved: crate::shared_atomic_usize_arc::SharedAtomicUsizeArc,
}

impl ResourceBudget {
    #[must_use]
    pub fn new(
        resource_budget_maximum: crate::resource_budget_maximum::ResourceBudgetMaximum,
    ) -> Self {
        Self {
            maximum: resource_budget_maximum,
            reserved: crate::shared_atomic_usize_arc::SharedAtomicUsizeArc::from(
                std::sync::Arc::from(std::sync::atomic::AtomicUsize::new(constants_usize::ZERO)),
            ),
        }
    }

    pub fn reserve(
        &self,
        resource_budget_amount: crate::resource_budget_amount::ResourceBudgetAmount,
    ) -> Result<
        crate::resource_budget_reservation::ResourceBudgetReservation,
        crate::resource_budget_reserve_error::ResourceBudgetReserveError,
    > {
        let result = self.reserved.try_update(
            std::sync::atomic::Ordering::AcqRel,
            std::sync::atomic::Ordering::Acquire,
            |current| {
                current
                    .checked_add(*resource_budget_amount)
                    .filter(|next| *next <= self.maximum.get())
            },
        );
        match result {
            Ok(_previous) => Ok(
                crate::resource_budget_reservation::ResourceBudgetReservation::new(
                    resource_budget_amount,
                    self.reserved.clone(),
                ),
            ),
            Err(current) if current.checked_add(*resource_budget_amount).is_none() => {
                Err(crate::resource_budget_reserve_error::ResourceBudgetReserveError::Overflow)
            }
            Err(_current) => {
                Err(crate::resource_budget_reserve_error::ResourceBudgetReserveError::Exhausted)
            }
        }
    }

    #[must_use]
    pub fn reserved(&self) -> crate::resource_budget_amount::ResourceBudgetAmount {
        crate::resource_budget_amount::ResourceBudgetAmount::from(
            self.reserved.load(std::sync::atomic::Ordering::Acquire),
        )
    }
}
