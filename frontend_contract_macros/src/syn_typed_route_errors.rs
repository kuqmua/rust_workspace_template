#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum SynTypedRouteErrors {
    Policy(crate::syn_expr::SynExpr),
    Statuses(crate::syn_expr::SynExpr),
}
