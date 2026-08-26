pub(super) fn success_redirect_impl(
    path: server_admin_contract::domain_types::AdminFrontendPath,
) -> axum::response::Response {
    axum::response::IntoResponse::into_response(axum::response::Redirect::to(
        format!("{}{}", path.get(), constants_str::ADMIN_HTML_SAVED_FRAGMENT).as_str(),
    ))
}
