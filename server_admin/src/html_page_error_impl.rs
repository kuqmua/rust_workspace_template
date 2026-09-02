pub(crate) fn html_page_error_impl(
    admin_error: crate::admin_error::AdminError,
) -> axum::response::Response {
    if matches!(admin_error, crate::admin_error::AdminError::Authentication) {
        axum::response::IntoResponse::into_response(axum::response::Redirect::to(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::SignIn.get(),
        ))
    } else {
        axum::response::IntoResponse::into_response(admin_error)
    }
}
