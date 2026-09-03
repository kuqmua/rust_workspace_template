#[allow(
    clippy::single_call_fn,
    reason = "frontend fallback routes remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) fn frontend_fallback_routes() -> server_runtime_http::axum_router::AxumRouter {
    server_runtime_http::axum_router::AxumRouter::from(axum::Router::new().fallback(async || {
        axum::response::Redirect::to(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::SignIn.get(),
        )
    }))
}
