pub(crate) fn admin_permission_requirement(
    permission: crate::admin_permission::AdminPermission,
) -> frontend_contract::authentication_requirement::AuthenticationRequirement {
    frontend_contract::authentication_requirement::AuthenticationRequirement::Permission(
        frontend_contract::contract_str::ContractStr::from(permission.as_str().get()),
    )
}
