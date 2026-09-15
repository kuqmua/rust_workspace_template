#[allow(
    clippy::single_call_fn,
    reason = "the generated route registry requires a named administrator OpenAPI handler"
)]
#[proc_macro_frontend_contract_route_operation::route_operation]
pub(crate) async fn admin_open_api() -> axum::Json<utoipa::openapi::OpenApi> {
    axum::Json(utoipa::openapi::OpenApi::from(
        server_admin::generated_open_api::generated_open_api(),
    ))
}
