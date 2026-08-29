pub(crate) async fn authorize_custom(
    auth: &crate::admin_auth_req::AdminAuthReq,
    permission: server_admin_contract::admin_permission::AdminPermission,
) -> Result<crate::authenticated_admin::AuthenticatedAdmin, crate::admin_error::AdminError> {
    crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        auth.state.as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(auth.headers.as_ref()),
        auth.peer,
        permission.as_str(),
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    )
    .await
}
