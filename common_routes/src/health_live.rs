#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn health_live() -> Result<
    crate::json_response::JsonResponse<crate::health_report::HealthReport>,
    crate::health_error::HealthError,
> {
    super::health_report_response::health_report_response(
        crate::health_report::HealthReport::liveness(),
    )
    .ok_or(crate::health_error::HealthError::Unavailable)
}
