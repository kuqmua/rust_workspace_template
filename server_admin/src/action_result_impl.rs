pub(crate) fn action_result_impl(
    result: Result<crate::axum_admin_response::AxumAdminResponse, crate::admin_error::AdminError>,
    admin_frontend_path: server_admin_contract::admin_frontend_path::AdminFrontendPath,
) -> axum::response::Response {
    match result {
        Ok(_response) => crate::success_redirect_impl::success_redirect_impl(admin_frontend_path),
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}
