pub(crate) fn authenticated_admin_contract(
    value: &crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin,
) -> Result<
    server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    crate::admin_error::AdminError,
> {
    let permissions = value
        .permissions
        .as_ref()
        .iter()
        .map(|permission| {
            server_admin_contract::admin_permission_value::AdminPermissionValue::try_from(
                permission.as_str().as_ref().to_owned(),
            )
            .map_err(|_error| crate::admin_error::AdminError::Validation)
        })
        .collect::<Result<Vec<_>, crate::admin_error::AdminError>>()?;
    let roles = value
        .roles
        .as_ref()
        .iter()
        .map(|role| {
            server_admin_contract::admin_role_name::AdminRoleName::try_from(
                role.as_ref().to_owned(),
            )
            .map_err(|_error| crate::admin_error::AdminError::Validation)
        })
        .collect::<Result<Vec<_>, crate::admin_error::AdminError>>()?;
    Ok(
        server_admin_contract::authenticated_admin::AuthenticatedAdmin::new(
            server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                value.display_name.as_ref().to_owned(),
            )
            .map_err(|_error| crate::admin_error::AdminError::Validation)?,
            server_admin_contract::admin_user_id::AdminUserId::from(value.id.value()),
            server_admin_contract::admin_login::AdminLogin::try_from(
                value.login.as_ref().to_owned(),
            )
            .map_err(|_error| crate::admin_error::AdminError::Validation)?,
            server_admin_contract::admin_permission_values::AdminPermissionValues::try_from(
                permissions,
            )
            .map_err(|_error| crate::admin_error::AdminError::Validation)?,
            server_admin_contract::admin_role_names::AdminRoleNames::try_from(roles)
                .map_err(|_error| crate::admin_error::AdminError::Validation)?,
        ),
    )
}
