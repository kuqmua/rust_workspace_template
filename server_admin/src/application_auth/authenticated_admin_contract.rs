use super::{AdminError, AuthenticatedAdmin};

pub(super) fn authenticated_admin_contract(
    value: &AuthenticatedAdmin,
) -> Result<server_admin_contract::domain_types::AuthenticatedAdmin, AdminError> {
    let permissions = value
        .permissions
        .as_ref()
        .iter()
        .map(|permission| {
            server_admin_contract::domain_types::AdminPermissionValue::try_from(
                permission.as_str().as_ref().to_owned(),
            )
            .map_err(|_error| AdminError::Validation)
        })
        .collect::<Result<Vec<_>, AdminError>>()?;
    let roles = value
        .roles
        .as_ref()
        .iter()
        .map(|role| {
            server_admin_contract::domain_types::AdminRoleName::try_from(role.as_ref().to_owned())
                .map_err(|_error| AdminError::Validation)
        })
        .collect::<Result<Vec<_>, AdminError>>()?;
    Ok(
        server_admin_contract::domain_types::AuthenticatedAdmin::new(
            server_admin_contract::domain_types::AdminDisplayName::try_from(
                value.display_name.as_ref().to_owned(),
            )
            .map_err(|_error| AdminError::Validation)?,
            server_admin_contract::domain_types::AdminUserId::from(value.id.value()),
            server_admin_contract::domain_types::AdminLogin::try_from(
                value.login.as_ref().to_owned(),
            )
            .map_err(|_error| AdminError::Validation)?,
            server_admin_contract::domain_types::AdminPermissionValues::try_from(permissions)
                .map_err(|_error| AdminError::Validation)?,
            server_admin_contract::domain_types::AdminRoleNames::try_from(roles)
                .map_err(|_error| AdminError::Validation)?,
        ),
    )
}
