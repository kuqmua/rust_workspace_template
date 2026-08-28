use crate::domain_types::{SynExpr, SynType, SynTypedRouteErrors};

#[derive(optimal_memory_layout::OptimalMemoryLayout, generate_accessor::Getters)]
#[getters(get_mut)]
pub(crate) struct TypedRouteArgs {
    authentication: SynExpr,
    error_response: Option<SynType>,
    errors: SynTypedRouteErrors,
    method: SynExpr,
    mutation: Option<SynExpr>,
    obligations: Option<SynExpr>,
    openapi_operation_id: SynExpr,
    path: SynExpr,
    path_parameter: Option<SynType>,
    request: SynType,
    request_body: Option<SynExpr>,
    response: SynType,
    success_status: SynExpr,
    transport: SynType,
}
#[allow(
    dead_code,
    reason = "field access is intentionally encapsulated behind uniform getters"
)]
impl TypedRouteArgs {
    #[allow(
        clippy::single_call_fn,
        clippy::too_many_arguments,
        reason = "constructor mirrors the parsed field model"
    )]
    pub(crate) const fn new(
        authentication: SynExpr,
        error_response: Option<SynType>,
        errors: SynTypedRouteErrors,
        method: SynExpr,
        mutation: Option<SynExpr>,
        obligations: Option<SynExpr>,
        openapi_operation_id: SynExpr,
        path: SynExpr,
        path_parameter: Option<SynType>,
        request: SynType,
        request_body: Option<SynExpr>,
        response: SynType,
        success_status: SynExpr,
        transport: SynType,
    ) -> Self {
        Self {
            authentication,
            error_response,
            errors,
            method,
            mutation,
            obligations,
            openapi_operation_id,
            path,
            path_parameter,
            request,
            request_body,
            response,
            success_status,
            transport,
        }
    }
}
