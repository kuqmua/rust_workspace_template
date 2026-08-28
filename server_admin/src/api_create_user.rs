// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract::route_openapi(delegate = crate::user_mutations_create::user_mutations_create, tag = "admin_users")]
pub(crate) async fn api_create_user(
    auth: crate::AdminAuthReq,
    request: crate::AxumAdminJson<server_admin_contract::domain_types::AdminCreateUserReq>,
) -> Result<crate::AxumAdminResponse, crate::AdminCreateUserError> {
}
