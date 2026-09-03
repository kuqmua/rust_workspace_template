#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) enum SynTypedRouteErrors {
    Policy(crate::contract_syn_expr::ContractSynExpr),
    Statuses(crate::contract_syn_expr::ContractSynExpr),
}
