#[frontend_contract_macros::route_operation]
#[allow(clippy::single_call_fn)] // named route or composition boundary has one registry or orchestration owner
pub(crate) async fn root() -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        server_admin_contract::admin_frontend_path::AdminFrontendPath::Users.get(),
    ))
}

// Root-owned module compatibility wrappers.
