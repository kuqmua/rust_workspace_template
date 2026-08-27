#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[must_use]
pub struct ResourceBudgetReservation {
    pub(super) amount: super::ResourceBudgetAmount,
    pub(super) reserved: super::SharedAtomicUsizeArc,
}

impl Drop for ResourceBudgetReservation {
    fn drop(&mut self) {
        let _previous = self
            .reserved
            .0
            .fetch_sub(self.amount.0, std::sync::atomic::Ordering::AcqRel);
    }
}
