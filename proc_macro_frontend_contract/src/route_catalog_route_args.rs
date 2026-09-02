#[derive(proc_macro_optimal_memory_layout::OptimalMemoryLayout, proc_macro_getters::Getters)]
#[getters(get_mut)]
pub(crate) struct RouteCatalogRouteArgs {
    contract: Option<crate::contract_syn_expr::ContractSynExpr>,
    path: Option<crate::contract_syn_expr::ContractSynExpr>,
    route: Option<crate::contract_syn_type::ContractSynType>,
    exclude_from_family: crate::std_bool::StdBool,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl RouteCatalogRouteArgs {
    #[allow(
        clippy::too_many_arguments,
        reason = "constructor mirrors the parsed field model"
    )]
    pub(crate) const fn new(
        contract: Option<crate::contract_syn_expr::ContractSynExpr>,
        path: Option<crate::contract_syn_expr::ContractSynExpr>,
        route: Option<crate::contract_syn_type::ContractSynType>,
        exclude_from_family: crate::std_bool::StdBool,
    ) -> Self {
        Self {
            contract,
            path,
            route,
            exclude_from_family,
        }
    }
}
