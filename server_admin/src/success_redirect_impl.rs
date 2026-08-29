pub(crate) fn success_redirect_impl(
    path: server_admin_contract::admin_frontend_path::AdminFrontendPath,
) -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        format!("{}{}", path.get(), constants_str::ADMIN_HTML_SAVED_FRAGMENT).as_str(),
    ))
}
