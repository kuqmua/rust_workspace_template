pub(super) fn action_result(
    result: Result<super::super::AxumAdminResponse, super::super::AdminError>,
    path: server_admin_contract::domain_types::AdminFrontendPath,
) -> axum::response::Response {
    match result {
        Ok(_response) => super::success_redirect_impl::success_redirect(path),
        Err(error) => axum::response::IntoResponse::into_response(error),
    }
}
