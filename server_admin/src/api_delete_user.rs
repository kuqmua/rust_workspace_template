// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::domain_types::route_openapi(delegate = crate::user_mutations_delete::user_mutations_delete, tag = "admin_users")]
pub(crate) async fn api_delete_user(
    auth: crate::AdminAuthReq,
    path: crate::AxumAdminPath<crate::AdminUserId>,
) -> Result<crate::AxumAdminResponse, crate::AdminDeleteUserError> {
}
