#[proc_macro_frontend_contract_route_openapi::route_openapi(tag = "service")]
#[allow(
    clippy::single_call_fn,
    reason = "route registry owns this Axum handler"
)]
pub(super) async fn health_live()
-> crate::json_response::JsonResponse<crate::health_report::HealthReport> {
    crate::make_json_response::make_json_response(crate::health_report::HealthReport::liveness())
}
