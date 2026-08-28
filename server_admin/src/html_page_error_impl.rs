pub(crate) fn html_page_error_impl(error: crate::AdminError) -> axum::response::Response {
    if matches!(error, crate::AdminError::Authentication) {
        axum::response::IntoResponse::into_response(axum::response::Redirect::to(
            server_admin_contract::domain_types::AdminFrontendPath::SignIn.get(),
        ))
    } else {
        axum::response::IntoResponse::into_response(error)
    }
}
