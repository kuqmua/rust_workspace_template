pub(crate) async fn account_me_context_view_ref(
    admin_auth_request: &crate::admin_auth_request::AdminAuthRequest,
) -> Result<
    (
        server_admin_contract::authenticated_admin::AuthenticatedAdmin,
        crate::admin_password_change_required::AdminPasswordChangeRequired,
    ),
    crate::admin_error::AdminError,
> {
    crate::authorization_authenticate::authorization_authenticate(
        admin_auth_request.get_state().as_ref(),
        crate::http_admin_header_map_ref::HttpAdminHeaderMapRef::from(
            admin_auth_request.get_headers().as_ref(),
        ),
        *admin_auth_request.get_peer(),
    )
    .await
    .and_then(|authenticated| {
        let password_change_required = authenticated.password_change_required();
        crate::authenticated_admin_contract::authenticated_admin_contract(&authenticated)
            .map(|contract| (contract, password_change_required))
    })
}
