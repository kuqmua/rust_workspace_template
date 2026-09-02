pub(crate) async fn authorization_authorize_generated_request(
    admin_auth_svc_state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    http_admin_header_map_ref: crate::http_admin_header_map_ref::HttpAdminHeaderMapRef<'_>,
    admin_peer_addr: crate::admin_peer_addr::AdminPeerAddr,
    admin_permission_str_ref: server_admin_contract::admin_permission_str_ref::AdminPermissionStrRef<'_>,
    std_admin_bool: server_admin_core::std_admin_bool::StdAdminBool,
) -> Result<
    crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin,
    crate::admin_error::AdminError,
> {
    let authenticated = crate::authorization_authenticate::authorization_authenticate(
        admin_auth_svc_state,
        http_admin_header_map_ref,
        admin_peer_addr,
    )
    .await?;
    if **authenticated.get_password_change_required() {
        return Err(crate::admin_error::AdminError::Authorization);
    }
    let required_permission = server_admin_contract::admin_permission::AdminPermission::try_from(
        admin_permission_str_ref.as_ref(),
    )
    .map_err(|_error| crate::admin_error::AdminError::Authorization)?;
    if !authenticated
        .get_permissions()
        .as_ref()
        .contains(&required_permission)
    {
        return Err(crate::admin_error::AdminError::Authorization);
    }
    if std_admin_bool.get() {
        let subject = server_admin_core::std_admin_string::StdAdminString::try_from(
            authenticated.get_id().get().to_string(),
        )
        .map_err(|_error| crate::admin_error::AdminError::Validation)?;
        crate::enforce_rate_limit::enforce_rate_limit(
            admin_auth_svc_state,
            crate::admin_rate_limit_scope::AdminRateLimitScope::Mutation,
            &subject,
            admin_auth_svc_state.get_policy().get_mutation_limit(),
            admin_auth_svc_state.get_policy().get_mutation_window(),
        )
        .await?;
        crate::authorization_validate_csrf::authorization_validate_csrf(
            admin_auth_svc_state,
            http_admin_header_map_ref,
            &authenticated,
        )
        .await?;
    }
    Ok(authenticated)
}
