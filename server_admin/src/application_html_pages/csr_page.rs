pub(super) async fn csr_page(
    auth: super::super::super::AdminAuthReq,
    page: server_admin_contract::domain_types::AdminPage,
    active_table: Option<server_admin_contract::domain_types::AdminDataTable>,
) -> axum::response::Response {
    match super::super::page_context_impl::page_context_impl(&auth).await {
        Ok((_admin, _branding, password_change_required))
            if *password_change_required
                && page != server_admin_contract::domain_types::AdminPage::Profile =>
        {
            axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                server_admin_contract::domain_types::AdminFrontendPath::Profile.get(),
            ))
        }
        Ok((admin, branding, _password_change_required))
            if bool::from(admin.can_access(page))
                && active_table
                    .is_none_or(|table| bool::from(admin.has_permission(table.permission()))) =>
        {
            super::super::html_response_impl::html_response_impl(
                server_admin_frontend::domain_types::ssr::render_admin_csr(
                    page,
                    active_table,
                    &admin,
                    &branding,
                ),
            )
        }
        Ok(_context) => super::super::html_page_error_impl::html_page_error_impl(
            super::super::super::AdminError::Authorization,
        ),
        Err(error) => super::super::html_page_error_impl::html_page_error_impl(error),
    }
}
