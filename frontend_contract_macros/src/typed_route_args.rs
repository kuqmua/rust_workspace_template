use super::{SynExpr, SynType, SynTypedRouteErrors};

#[derive(optimal_memory_layout::OptimalMemoryLayout)]
pub(crate) struct TypedRouteArgs {
    pub(crate) authentication: SynExpr,
    pub(crate) error_response: Option<SynType>,
    pub(crate) errors: SynTypedRouteErrors,
    pub(crate) method: SynExpr,
    pub(crate) mutation: Option<SynExpr>,
    pub(crate) obligations: Option<SynExpr>,
    pub(crate) openapi_operation_id: SynExpr,
    pub(crate) path: SynExpr,
    pub(crate) path_parameter: Option<SynType>,
    pub(crate) request: SynType,
    pub(crate) request_body: Option<SynExpr>,
    pub(crate) response: SynType,
    pub(crate) success_status: SynExpr,
    pub(crate) transport: SynType,
}
