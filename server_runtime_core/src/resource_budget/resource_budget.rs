#[derive(optimal_memory_layout::OptimalMemoryLayout, Clone, Debug)]
pub struct ResourceBudget {
    maximum: super::ResourceBudgetMaximum,
    reserved: super::SharedAtomicUsizeArc,
}

impl ResourceBudget {
    #[must_use]
    pub fn new(maximum: super::ResourceBudgetMaximum) -> Self {
        Self {
            maximum,
            reserved: super::SharedAtomicUsizeArc::from(std::sync::Arc::from(
                std::sync::atomic::AtomicUsize::new(constants_usize::ZERO),
            )),
        }
    }

    pub fn reserve(
        &self,
        amount: super::ResourceBudgetAmount,
    ) -> Result<super::ResourceBudgetReservation, super::ResourceBudgetReserveError> {
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
            Ok(_previous) => Ok(super::ResourceBudgetReservation {
                amount,
                reserved: self.reserved.clone(),
            }),
            Err(current) if current.checked_add(amount.0).is_none() => {
                Err(super::ResourceBudgetReserveError::Overflow)
            }
            Err(_current) => Err(super::ResourceBudgetReserveError::Exhausted),
        }
    }

    #[must_use]
    pub fn reserved(&self) -> super::ResourceBudgetAmount {
        super::ResourceBudgetAmount::from(
            self.reserved.0.load(std::sync::atomic::Ordering::Acquire),
        )
    }
}
