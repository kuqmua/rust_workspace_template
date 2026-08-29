pub(crate) async fn crud_page<View, Load, LoadFuture, Render>(
    auth: crate::admin_auth_req::AdminAuthReq,
    permissions: &[server_admin_contract::admin_permission::AdminPermission],
    load: Load,
    render: Render,
) -> axum::response::Response
where
    Load: FnOnce(crate::admin_auth_req::AdminAuthReq) -> LoadFuture,
    LoadFuture: Future<Output = Result<View, crate::admin_error::AdminError>>,
    Render: FnOnce(
        &View,
        &server_admin_contract::authenticated_admin::AuthenticatedAdmin,
        &server_admin_contract::admin_branding_view::AdminBrandingView,
    ) -> server_admin_frontend::admin_ssr_html::AdminSsrHtml,
{
    match crate::page_context_impl::page_context_impl(&auth).await {
        Ok((_admin, _branding, password_change_required)) if *password_change_required => {
            axum::response::IntoResponse::into_response(axum::response::Redirect::to(
                server_admin_contract::admin_frontend_path::AdminFrontendPath::Profile.get(),
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
        Ok(_context) => crate::html_page_error_impl::html_page_error_impl(
            crate::admin_error::AdminError::Authorization,
        ),
        Err(error) => crate::html_page_error_impl::html_page_error_impl(error),
    }
}
