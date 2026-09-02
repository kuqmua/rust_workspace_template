pub(crate) fn admin_permission_requirement(
    admin_permission: crate::admin_permission::AdminPermission,
) -> frontend_contract::authentication_requirement::AuthenticationRequirement {
    frontend_contract::authentication_requirement::AuthenticationRequirement::Permission(
        frontend_contract::contract_str::ContractStr::from(admin_permission.as_str().get()),
    )
}
