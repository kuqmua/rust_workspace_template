pub(crate) fn action_result_impl(
    result: Result<crate::AxumAdminResponse, crate::AdminError>,
    path: server_admin_contract::domain_types::AdminFrontendPath,
) -> axum::response::Response {
    match result {
        Ok(_response) => crate::success_redirect_impl::success_redirect_impl(path),
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}
