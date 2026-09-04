#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_getters::Getters)]
#[getters(get_mut)]
#[derive(proc_macro_new::New)]
pub(crate) struct RouteCatalogRouteArgs {
    contract: Option<crate::contract_syn_expr::ContractSynExpr>,
    path: Option<crate::contract_syn_expr::ContractSynExpr>,
    route: Option<crate::contract_syn_type::ContractSynType>,
    exclude_from_family: crate::std_bool::StdBool,
}
