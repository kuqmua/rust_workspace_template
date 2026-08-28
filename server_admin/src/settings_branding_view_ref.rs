pub(crate) async fn settings_branding_view_ref(
    auth: &crate::AdminAuthReq,
) -> Result<server_admin_contract::domain_types::AdminBrandingView, crate::AdminError> {
    let settings = crate::repository::read_settings::read_settings(
        crate::repository::SqlxAdminRepositoryPoolRef::from(auth.state.as_ref().pool.as_ref()),
    )
    .await
    .map_err(crate::shared::map_repository_error::map_repository_error)?;
    Ok(server_admin_contract::domain_types::AdminBrandingView::from_settings(&settings))
}
