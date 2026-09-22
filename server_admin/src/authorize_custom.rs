pub(crate) async fn authorize_custom(
    admin_auth_request: &crate::admin_auth_request::AdminAuthRequest,
    admin_rule: server_admin_contract::admin_rule::AdminRule,
) -> Result<
    crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin,
    crate::admin_error::AdminError,
> {
    crate::authorization_authorize_generated_request::authorization_authorize_generated_request(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(
            admin_auth_request.get_headers().as_ref(),
        ),
        *admin_auth_request.get_peer(),
        admin_rule.as_str(),
        server_admin_core::std_admin_bool::StdAdminBool::from(true),
    )
    .await
}
