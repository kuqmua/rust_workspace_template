pub(crate) async fn authorization_authorize_generated_request(
    state: &super::AdminAuthSvcState,
    headers: super::super::HttpAdminHeaderMapRef<'_>,
    peer: super::AdminPeerAddr,
    permission: server_admin_contract::domain_types::AdminPermissionStrRef<'_>,
    mutates: super::super::StdAdminBool,
) -> Result<super::AuthenticatedAdmin, super::AdminError> {
    let authenticated =
        super::authorization_authenticate::authorization_authenticate(state, headers, peer).await?;
    if *authenticated.password_change_required {
        return Err(super::AdminError::Authorization);
    }
    let required_permission = super::super::AdminPermission::try_from(permission.as_ref())
        .map_err(|_error| super::AdminError::Authorization)?;
    if !authenticated
        .permissions
        .as_ref()
        .contains(&required_permission)
    {
        return Err(super::AdminError::Authorization);
    }
    if mutates.get() {
        let subject = super::super::StdAdminString::try_from(authenticated.id.get().to_string())
            .map_err(|_error| super::AdminError::Validation)?;
        super::rate_limit::enforce_rate_limit(
            state,
            super::rate_limit::AdminRateLimitScope::Mutation,
            &subject,
            state.policy.mutation_limit,
            state.policy.mutation_window,
        )
        .await?;
        super::authorization_validate_csrf::authorization_validate_csrf(
            state,
            headers,
            &authenticated,
        )
        .await?;
    }
    Ok(authenticated)
}
