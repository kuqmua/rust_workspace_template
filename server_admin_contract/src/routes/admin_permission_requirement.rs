pub(in crate::domain_types) fn admin_permission_requirement(
    permission: crate::domain_types::AdminPermission,
) -> frontend_contract::domain_types::AuthenticationRequirement {
    frontend_contract::domain_types::AuthenticationRequirement::Permission(
        frontend_contract::domain_types::ContractStr::from(permission.as_str().get()),
    )
}
