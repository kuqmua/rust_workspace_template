#[proc_macro_frontend_contract::route_operation]
#[allow(
    clippy::single_call_fn,
    reason = "root remains a named owner because its boundary role is clearer and directly testable"
)]
pub(crate) async fn root() -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
    ))
}
