use super::{SynExpr, SynType, SynTypedRouteErrors};

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct TypedRouteArgs {
    pub authentication: SynExpr,
    pub error_response: Option<SynType>,
    pub errors: SynTypedRouteErrors,
    pub method: SynExpr,
    pub mutation: Option<SynExpr>,
    pub obligations: Option<SynExpr>,
    pub openapi_operation_id: SynExpr,
    pub path: SynExpr,
    pub path_parameter: Option<SynType>,
    pub request: SynType,
    pub request_body: Option<SynExpr>,
    pub response: SynType,
    pub success_status: SynExpr,
    pub transport: SynType,
}
