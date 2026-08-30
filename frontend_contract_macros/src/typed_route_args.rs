#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct TypedRouteArgs {
    pub authentication: crate::contract_syn_expr::ContractSynExpr,
    pub error_response: Option<crate::contract_syn_type::ContractSynType>,
    pub errors: crate::syn_typed_route_errors::SynTypedRouteErrors,
    pub method: crate::contract_syn_expr::ContractSynExpr,
    pub mutation: Option<crate::contract_syn_expr::ContractSynExpr>,
    pub obligations: Option<crate::contract_syn_expr::ContractSynExpr>,
    pub openapi_operation_id: crate::contract_syn_expr::ContractSynExpr,
    pub path: crate::contract_syn_expr::ContractSynExpr,
    pub path_parameter: Option<crate::contract_syn_type::ContractSynType>,
    pub request: crate::contract_syn_type::ContractSynType,
    pub request_body: Option<crate::contract_syn_expr::ContractSynExpr>,
    pub response: crate::contract_syn_type::ContractSynType,
    pub success_status: crate::contract_syn_expr::ContractSynExpr,
    pub transport: crate::contract_syn_type::ContractSynType,
}
