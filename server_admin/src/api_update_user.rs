// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::domain_types::route_openapi(delegate = crate::user_mutations_update::user_mutations_update, tag = "admin_users")]
pub(crate) async fn api_update_user(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminUserId>,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminUpdateUserReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminUpdateUserError> {
}
