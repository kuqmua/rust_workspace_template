pub(crate) async fn authorization_authorize_generated_request(
    state: &crate::AdminAuthSvcState,
    headers: crate::HttpAdminHeaderMapRef<'_>,
    peer: crate::AdminPeerAddr,
    permission: server_admin_contract::domain_types::AdminPermissionStrRef<'_>,
    mutates: crate::StdAdminBool,
) -> Result<crate::AuthenticatedAdmin, crate::AdminError> {
    let authenticated =
        crate::authorization_authenticate::authorization_authenticate(state, headers, peer).await?;
    if *authenticated.password_change_required {
        return Err(crate::AdminError::Authorization);
    }
    let required_permission = crate::AdminPermission::try_from(permission.as_ref())
        .map_err(|_error| crate::AdminError::Authorization)?;
    if !authenticated
        .permissions
        .as_ref()
        .contains(&required_permission)
    {
        return Err(crate::AdminError::Authorization);
    }
    if mutates.get() {
        let subject = crate::StdAdminString::try_from(authenticated.id.get().to_string())
            .map_err(|_error| crate::AdminError::Validation)?;
        crate::rate_limit::enforce_rate_limit(
            state,
            crate::rate_limit::AdminRateLimitScope::Mutation,
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
