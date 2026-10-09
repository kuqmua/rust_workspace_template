pub(crate) fn authenticated_admin_contract(
    runtime_authenticated_admin: &crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin,
) -> Result<
    server_admin_contract::authenticated_admin::AuthenticatedAdmin,
    crate::admin_error::AdminError,
> {
    let rules = runtime_authenticated_admin
        .get_rules()
        .as_ref()
        .iter()
        .map(|rule| {
            server_admin_contract::admin_rule_value::AdminRuleValue::try_from(
                rule.as_str().as_ref().to_owned(),
            )
            .map_err(|_error| crate::admin_error::AdminError::Validation)
        })
        .collect::<Result<Vec<_>, crate::admin_error::AdminError>>()?;
    let roles = runtime_authenticated_admin
        .get_roles()
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
                runtime_authenticated_admin
                    .get_display_name()
                    .as_ref()
                    .to_owned(),
            )
            .map_err(|_error| crate::admin_error::AdminError::Validation)?,
            server_admin_contract::admin_user_id::AdminUserId::from(
                runtime_authenticated_admin.get_id().value(),
            ),
            server_admin_contract::admin_login::AdminLogin::try_from(
                runtime_authenticated_admin.get_login().as_ref().to_owned(),
            )
            .map_err(|_error| crate::admin_error::AdminError::Validation)?,
            server_admin_contract::admin_rule_values::AdminRuleValues::try_from(rules)
                .map_err(|_error| crate::admin_error::AdminError::Validation)?,
            server_admin_contract::admin_role_names::AdminRoleNames::try_from(roles)
                .map_err(|_error| crate::admin_error::AdminError::Validation)?,
        ),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_authenticated_admin_projection_preserves_identity_and_bounded_collection_order() {
        assert!(
            [
                0usize,
                3usize,
                crate::admin_auth_collection_max_len::ADMIN_AUTH_COLLECTION_MAX_LEN
            ]
            .into_iter()
            .all(|count| {
                let rule_values = [
                    server_admin_contract::admin_rule::AdminRule::MetricsRead,
                    server_admin_contract::admin_rule::AdminRule::UsersRead,
                ];
                let role_values = [constants_str::USER, constants_str::LOGIN];
                let roles = role_values
                    .into_iter()
                    .cycle()
                    .take(count)
                    .map(|role| {
                        server_admin_contract::admin_role_name::AdminRoleName::try_from(
                            role.to_owned(),
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|error| error.to_string())
                    .and_then(|names| {
                        crate::runtime_admin_role_names::RuntimeAdminRoleNames::try_from(names)
                            .map_err(|error| error.to_string())
                    });
                let (
                    Ok(display_name),
                    Ok(login),
                    Ok(identifier),
                    Ok(runtime_rules),
                    Ok(runtime_roles),
                ) = (
                    server_admin_contract::admin_display_name::AdminDisplayName::try_from(
                        constants_str::ADMIN.to_owned(),
                    ),
                    server_admin_contract::admin_login::AdminLogin::try_from(
                        constants_str::LOGIN.to_owned(),
                    ),
                    server_admin_core::admin_user_record_id::AdminUserRecordId::try_from(1i64),
                    crate::admin_auth_rules::AdminAuthRules::try_from(
                        rule_values
                            .into_iter()
                            .cycle()
                            .take(count)
                            .collect::<Vec<_>>(),
                    ),
                    roles,
                )
                else {
                    return false;
                };
                let runtime_admin =
                    crate::runtime_authenticated_admin::RuntimeAuthenticatedAdmin::new(
                        display_name,
                        identifier,
                        login,
                        runtime_rules,
                        runtime_roles,
                        crate::admin_session_id::AdminSessionId::from(identifier.value()),
                        crate::admin_password_change_required::AdminPasswordChangeRequired::from(
                            true,
                        ),
                    );
                super::authenticated_admin_contract(&runtime_admin).is_ok_and(|public_admin| {
                    public_admin.display_name().as_ref() == constants_str::ADMIN
                        && public_admin.login().as_ref() == constants_str::LOGIN
                        && public_admin.id().value() == identifier.value()
                        && public_admin.rules().len() == count
                        && public_admin.roles().len() == count
                        && public_admin
                            .rules()
                            .iter()
                            .zip(rule_values.into_iter().cycle())
                            .all(|(actual_rule, expected_rule)| {
                                actual_rule.as_ref() == expected_rule.as_str().as_ref()
                            })
                        && public_admin
                            .roles()
                            .iter()
                            .zip(role_values.into_iter().cycle())
                            .all(|(actual_role, expected_role)| {
                                actual_role.as_ref() == expected_role
                            })
                        && serde_json::to_value(&public_admin).is_ok_and(|payload| {
                            payload.as_object().is_some_and(|object| {
                                object.len() == 5usize
                                    && [
                                        constants_str::DISPLAY_NAME,
                                        constants_str::SQL_NAMES_ID,
                                        constants_str::LOGIN,
                                        constants_str::RULES_TABLE,
                                        constants_str::ROLES_TABLE,
                                    ]
                                    .into_iter()
                                    .all(|field| object.contains_key(field))
                            })
                        })
                })
            })
        );
    }
}
