pub(crate) fn html_page_error_impl(
    error: crate::admin_error::AdminError,
) -> axum::response::Response {
    if matches!(error, crate::admin_error::AdminError::Authentication) {
        axum::response::IntoResponse::into_response(axum::response::Redirect::to(
            server_admin_contract::admin_frontend_path::AdminFrontendPath::SignIn.get(),
        ))
    } else {
        axum::response::IntoResponse::into_response(error)
    }
}
