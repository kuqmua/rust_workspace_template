#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    proc_macro_getters::Getters,
    proc_macro_new::New,
)]
#[getters(get_mut)]
pub(crate) struct TypedRouteArgs {
    authentication: crate::contract_syn_expr::ContractSynExpr,
    error_response: Option<crate::contract_syn_type::ContractSynType>,
    errors: crate::syn_typed_route_errors::SynTypedRouteErrors,
    method: crate::contract_syn_expr::ContractSynExpr,
    mutation: Option<crate::contract_syn_expr::ContractSynExpr>,
    obligations: Option<crate::contract_syn_expr::ContractSynExpr>,
    openapi_operation_id: crate::contract_syn_expr::ContractSynExpr,
    path: crate::contract_syn_expr::ContractSynExpr,
    path_parameter: Option<crate::contract_syn_type::ContractSynType>,
    request: crate::contract_syn_type::ContractSynType,
    request_body: Option<crate::contract_syn_expr::ContractSynExpr>,
    response: crate::contract_syn_type::ContractSynType,
    success_status: crate::contract_syn_expr::ContractSynExpr,
    transport: crate::contract_syn_type::ContractSynType,
}
