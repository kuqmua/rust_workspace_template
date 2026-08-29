pub(crate) async fn authorization_authorize_generated_request(
    state: &crate::admin_auth_svc_state::AdminAuthSvcState,
    headers: crate::http_admin_header_map_ref::HttpAdminHeaderMapRef<'_>,
    peer: crate::admin_peer_addr::AdminPeerAddr,
    permission: server_admin_contract::admin_permission_str_ref::AdminPermissionStrRef<'_>,
    mutates: server_admin_core::std_admin_bool::StdAdminBool,
) -> Result<crate::authenticated_admin::AuthenticatedAdmin, crate::admin_error::AdminError> {
    let authenticated =
        crate::authorization_authenticate::authorization_authenticate(state, headers, peer).await?;
    if *authenticated.password_change_required {
        return Err(crate::admin_error::AdminError::Authorization);
    }
    let required_permission =
        server_admin_contract::admin_permission::AdminPermission::try_from(permission.as_ref())
            .map_err(|_error| crate::admin_error::AdminError::Authorization)?;
    if !authenticated
        .permissions
        .as_ref()
        .contains(&required_permission)
    {
        return Err(crate::admin_error::AdminError::Authorization);
    }
    if mutates.get() {
        let subject = server_admin_core::std_admin_string::StdAdminString::try_from(
            authenticated.id.get().to_string(),
        )
        .map_err(|_error| crate::admin_error::AdminError::Validation)?;
        crate::enforce_rate_limit::enforce_rate_limit(
            state,
            crate::admin_rate_limit_scope::AdminRateLimitScope::Mutation,
            &subject,
            state.policy.mutation_limit,
            state.policy.mutation_window,
        )
        .await?;
        crate::authorization_validate_csrf::authorization_validate_csrf(
            state,
            headers,
            &authenticated,
        )
        .await?;
    }
    Ok(authenticated)
}
