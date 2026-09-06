pub(crate) async fn csr_page(
    admin_auth_request: crate::admin_auth_request::AdminAuthRequest,
    admin_page: server_admin_contract::admin_page::AdminPage,
    option: Option<server_admin_contract::admin_data_table::AdminDataTable>,
) -> axum::response::Response {
    match crate::page_context_impl::page_context_impl(&admin_auth_request).await {
        Ok((_admin, _branding, password_change_required))
            if *password_change_required
                && admin_page != server_admin_contract::admin_page::AdminPage::Profile =>
        {
            axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile.get(),
            ))
        }
        Ok((admin, branding, password_change_required))
            if bool::from(admin.can_access(admin_page))
                && option
                    .is_none_or(|table| bool::from(admin.has_permission(table.permission()))) =>
        {
            crate::html_response_impl::html_response_impl(
                frontend_admin::render_admin_csr::render_admin_csr(
                    server_admin_contract::admin_bool::AdminBool::from(*password_change_required),
                    admin_page,
                    option,
                    &admin,
                    &branding,
                ),
            )
        }
        Ok(_context) => crate::html_page_error_impl::html_page_error_impl(
            crate::admin_error::AdminError::Authorization,
        ),
        Err(error) => crate::html_page_error_impl::html_page_error_impl(error),
    }
}
