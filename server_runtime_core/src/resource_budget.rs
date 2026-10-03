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

#[cfg(test)]
mod tests {
    #[test]
    fn test_budget_shares_reservations_and_recovers_capacity_after_drop() {
        let budget = super::ResourceBudget::new(
            crate::resource_budget_maximum::ResourceBudgetMaximum::from(
                std::num::NonZeroUsize::MIN.saturating_add(constants_usize::TWO),
            ),
        );
        let clone = budget.clone();
        let first = budget.reserve(crate::resource_budget_amount::ResourceBudgetAmount::from(
            constants_usize::TWO,
        ));
        assert!(
            first
                .as_ref()
                .is_ok_and(|_reservation| *clone.reserved() == constants_usize::TWO)
        );
        let second = clone.reserve(crate::resource_budget_amount::ResourceBudgetAmount::from(
            constants_usize::ONE,
        ));
        assert!(
            second
                .as_ref()
                .is_ok_and(|_reservation| *budget.reserved() == constants_usize::THREE)
        );
        assert!(
            budget
                .reserve(crate::resource_budget_amount::ResourceBudgetAmount::from(
                    constants_usize::ONE
                ))
                .is_err_and(|error| error
                    == crate::resource_budget_reserve_error::ResourceBudgetReserveError::Exhausted)
        );
        assert_eq!(*clone.reserved(), constants_usize::THREE);
        drop(first);
        assert_eq!(*budget.reserved(), constants_usize::ONE);
        drop(second);
        assert_eq!(*clone.reserved(), constants_usize::ZERO);
        let recovered = clone.reserve(crate::resource_budget_amount::ResourceBudgetAmount::from(
            constants_usize::THREE,
        ));
        assert!(
            recovered
                .as_ref()
                .is_ok_and(|_reservation| *budget.reserved() == constants_usize::THREE)
        );
        drop(recovered);
        assert_eq!(*budget.reserved(), constants_usize::ZERO);
    }

    #[test]
    fn test_budget_zero_reservation_leaves_full_capacity_unchanged() {
        let budget = super::ResourceBudget::new(
            crate::resource_budget_maximum::ResourceBudgetMaximum::from(
                std::num::NonZeroUsize::MIN,
            ),
        );
        let full = budget.reserve(crate::resource_budget_amount::ResourceBudgetAmount::from(
            constants_usize::ONE,
        ));
        assert!(
            full.as_ref()
                .is_ok_and(|_reservation| *budget.reserved() == constants_usize::ONE)
        );
        let zero = budget.reserve(crate::resource_budget_amount::ResourceBudgetAmount::from(
            constants_usize::ZERO,
        ));
        assert!(
            zero.as_ref()
                .is_ok_and(|_reservation| *budget.reserved() == constants_usize::ONE)
        );
        drop(zero);
        assert_eq!(*budget.reserved(), constants_usize::ONE);
        drop(full);
        assert_eq!(*budget.reserved(), constants_usize::ZERO);
    }

    #[test]
    fn test_budget_overflow_preserves_accounting_and_release() {
        let maximum = std::num::NonZeroUsize::MAX;
        let budget = super::ResourceBudget::new(
            crate::resource_budget_maximum::ResourceBudgetMaximum::from(maximum),
        );
        let full = budget.reserve(crate::resource_budget_amount::ResourceBudgetAmount::from(
            usize::from(maximum),
        ));
        assert!(
            full.as_ref()
                .is_ok_and(|_reservation| *budget.reserved() == usize::from(maximum))
        );
        assert!(
            budget
                .reserve(crate::resource_budget_amount::ResourceBudgetAmount::from(
                    constants_usize::ONE
                ))
                .is_err_and(|error| error
                    == crate::resource_budget_reserve_error::ResourceBudgetReserveError::Overflow)
        );
        assert_eq!(*budget.reserved(), usize::from(maximum));
        drop(full);
        assert_eq!(*budget.reserved(), constants_usize::ZERO);
    }
    #[test]
    fn test_budget_maximum_rejects_zero_and_preserves_positive_capacity() {
        assert!(
            crate::resource_budget_maximum::ResourceBudgetMaximum::try_from(constants_usize::ZERO)
                .is_err_and(|error| error
                    == crate::resource_budget_config_error::ResourceBudgetConfigError::Zero)
        );
        [constants_usize::ONE, constants_usize::THREE]
            .into_iter()
            .fold((), |(), maximum| {
                assert!(
                    crate::resource_budget_maximum::ResourceBudgetMaximum::try_from(maximum)
                        .is_ok_and(|value| value.get() == maximum)
                );
            });
    }
}
