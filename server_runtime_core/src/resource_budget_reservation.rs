#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug, generate_constructor::New)]
#[constructor(pub(crate))]
#[must_use]
pub struct ResourceBudgetReservation {
    amount: crate::resource_budget_amount::ResourceBudgetAmount,
    reserved: crate::shared_atomic_usize_arc::SharedAtomicUsizeArc,
}

impl Drop for ResourceBudgetReservation {
    fn drop(&mut self) {
        let _previous = self
            .reserved
            .fetch_sub(*self.amount, std::sync::atomic::Ordering::AcqRel);
    }
}
