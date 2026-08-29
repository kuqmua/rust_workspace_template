pub(crate) async fn csr_page(
    auth: crate::admin_auth_req::AdminAuthReq,
    page: server_admin_contract::admin_page::AdminPage,
    active_table: Option<server_admin_contract::admin_data_table::AdminDataTable>,
) -> axum::response::Response {
    match crate::page_context_impl::page_context_impl(&auth).await {
        Ok((_admin, _branding, password_change_required))
            if *password_change_required
                && page != server_admin_contract::admin_page::AdminPage::Profile =>
        {
            axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile.get(),
            ))
        }
        Ok((admin, branding, _password_change_required))
            if bool::from(admin.can_access(page))
                && active_table
                    .is_none_or(|table| bool::from(admin.has_permission(table.permission()))) =>
        {
            crate::html_response_impl::html_response_impl(
                server_admin_frontend::render_admin_csr::render_admin_csr(
                    page,
                    active_table,
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
