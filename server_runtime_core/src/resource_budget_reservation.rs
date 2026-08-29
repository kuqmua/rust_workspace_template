#![allow(
    clippy::field_scoped_visibility_modifiers,
    reason = "the owner-module split exposes representation only to its parent facade"
)]
#[derive(optimal_memory_layout::OptimalMemoryLayout, Debug)]
#[must_use]
pub struct ResourceBudgetReservation {
    pub(super) amount: crate::resource_budget_amount::ResourceBudgetAmount,
    pub(super) reserved: crate::shared_atomic_usize_arc::SharedAtomicUsizeArc,
}

impl Drop for ResourceBudgetReservation {
    fn drop(&mut self) {
        let _previous = self
            .reserved
            .0
            .fetch_sub(self.amount.0, std::sync::atomic::Ordering::AcqRel);
    }
}
