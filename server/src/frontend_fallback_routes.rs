#[allow(clippy::single_call_fn)] // named command or composition stage has one orchestration owner
pub(crate) fn frontend_fallback_routes() -> server_runtime_http::domain_types::AxumRouter {
    server_runtime_http::domain_types::AxumRouter::from(axum::Router::new().fallback(async || {
        axum::response::Redirect::to(
            server_admin_contract::domain_types::AdminFrontendPath::SignIn.get(),
        )
    }))
}
