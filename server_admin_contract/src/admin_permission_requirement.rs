pub(crate) fn admin_permission_requirement(
    permission: crate::domain_types::AdminPermission,
) -> frontend_contract::AuthenticationRequirement {
    frontend_contract::AuthenticationRequirement::Permission(frontend_contract::ContractStr::from(
        permission.as_str().get(),
    ))
}
