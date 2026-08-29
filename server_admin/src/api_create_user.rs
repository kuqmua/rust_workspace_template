// The owner module retains lint-sensitive semantics from the original implementation.

#[frontend_contract_macros::route_openapi(delegate = crate::user_mutations_create::user_mutations_create, tag = "admin_users")]
pub(crate) async fn api_create_user(
    auth: crate::admin_auth_req::AdminAuthReq,
    request: crate::axum_admin_json::AxumAdminJson<
        server_admin_contract::admin_create_user_req::AdminCreateUserReq,
    >,
) -> Result<
    crate::axum_admin_response::AxumAdminResponse,
    crate::application_auth::AdminCreateUserError,
> {
}
