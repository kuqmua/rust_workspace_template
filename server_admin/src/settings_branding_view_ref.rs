pub(crate) async fn settings_branding_view_ref(
    admin_auth_request: &crate::admin_auth_request::AdminAuthRequest,
) -> Result<
    server_admin_contract::admin_branding_view::AdminBrandingView,
    crate::admin_error::AdminError,
> {
    let settings = crate::read_settings::read_settings(
        crate::sqlx_admin_repository_pool_ref::SqlxAdminRepositoryPoolRef::from(
            admin_auth_request.get_state().as_ref().get_pool().as_ref(),
        ),
    )
    .await
    .map_err(crate::map_repository_error::map_repository_error)?;
    Ok(server_admin_contract::admin_branding_view::AdminBrandingView::from_settings(&settings))
}
