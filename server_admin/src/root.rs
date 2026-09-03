#[proc_macro_frontend_contract::route_operation]
#[allow(clippy::single_call_fn, reason = "lint suppression is required here")]
pub(crate) async fn root() -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
    ))
}
