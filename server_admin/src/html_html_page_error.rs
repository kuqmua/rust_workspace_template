pub(super) fn html_page_error(error: super::super::AdminError) -> axum::response::Response {
    if matches!(error, super::super::AdminError::Authentication) {
        axum::response::IntoResponse::into_response(axum::response::Redirect::to(
            server_admin_contract::domain_types::AdminFrontendPath::SignIn.get(),
        ))
    } else {
        axum::response::IntoResponse::into_response(error)
    }
}
