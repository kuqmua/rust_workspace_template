use super::SynExpr;

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum SynTypedRouteErrors {
    Policy(SynExpr),
    Statuses(SynExpr),
}
