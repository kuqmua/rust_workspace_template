#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct TypedRouteArgs {
    pub authentication: crate::syn_expr::SynExpr,
    pub error_response: Option<crate::syn_type::SynType>,
    pub errors: crate::syn_typed_route_errors::SynTypedRouteErrors,
    pub method: crate::syn_expr::SynExpr,
    pub mutation: Option<crate::syn_expr::SynExpr>,
    pub obligations: Option<crate::syn_expr::SynExpr>,
    pub openapi_operation_id: crate::syn_expr::SynExpr,
    pub path: crate::syn_expr::SynExpr,
    pub path_parameter: Option<crate::syn_type::SynType>,
    pub request: crate::syn_type::SynType,
    pub request_body: Option<crate::syn_expr::SynExpr>,
    pub response: crate::syn_type::SynType,
    pub success_status: crate::syn_expr::SynExpr,
    pub transport: crate::syn_type::SynType,
}
