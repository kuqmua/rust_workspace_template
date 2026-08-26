#![allow(clippy::single_call_fn)] // HTML composition owns the borrowed branding operation

pub(super) async fn settings_branding_view_ref(
    auth: &super::AdminAuthReq,
) -> Result<server_admin_contract::domain_types::AdminBrandingView, super::AdminError> {
    let settings = crate::adapters::repository::read_settings::read_settings(
        crate::adapters::repository::SqlxAdminRepositoryPoolRef::from(
            auth.state.as_ref().pool.as_ref(),
        ),
    )
    .await
    .map_err(super::shared::map_repository_error::map_repository_error)?;
    Ok(server_admin_contract::domain_types::AdminBrandingView::from_settings(&settings))
}
