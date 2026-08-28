pub(crate) async fn crud_page<View, Load, LoadFuture, Render>(
    auth: crate::AdminAuthReq,
    permissions: &[server_admin_contract::domain_types::AdminPermission],
    load: Load,
    render: Render,
) -> axum::response::Response
where
    Load: FnOnce(crate::AdminAuthReq) -> LoadFuture,
    LoadFuture: Future<Output = Result<View, crate::AdminError>>,
    Render: FnOnce(
        &View,
        &server_admin_contract::domain_types::AuthenticatedAdmin,
        &server_admin_contract::domain_types::AdminBrandingView,
    ) -> server_admin_frontend::domain_types::ssr::AdminSsrHtml,
{
    match crate::page_context_impl::page_context_impl(&auth).await {
        Ok((_admin, _branding, password_change_required)) if *password_change_required => {
            axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                server_admin_contract::domain_types::AdminFrontendPath::Profile.get(),
            ))
        }
        Ok((admin, branding, _password_change_required))
            if permissions
                .iter()
                .any(|permission| bool::from(admin.has_permission(*permission))) =>
        {
            match load(auth).await {
                Ok(view) => {
                    crate::html_response_impl::html_response_impl(render(&view, &admin, &branding))
                }
                Err(error) => crate::html_page_error_impl::html_page_error_impl(error),
            }
        }
        Ok(_context) => {
            crate::html_page_error_impl::html_page_error_impl(crate::AdminError::Authorization)
        }
        Err(error) => crate::html_page_error_impl::html_page_error_impl(error),
    }
}
