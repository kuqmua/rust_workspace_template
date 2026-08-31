pub(crate) async fn authorize_custom(
    auth: &crate::admin_auth_req::AdminAuthReq,
    permission: server_admin_contract::admin_permission::AdminPermission,
) -> Result<
    crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin,
    crate::admin_error::AdminError,
> {
    crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.get_headers().as_ref()),
        *auth.get_peer(),
        permission.as_str(),
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    )
    .await
}
