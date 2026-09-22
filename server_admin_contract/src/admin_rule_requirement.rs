pub(crate) fn admin_rule_requirement(
    admin_rule: crate::admin_rule::AdminRule,
) -> frontend_contract::authentication_requirement::AuthenticationRequirement {
    frontend_contract::authentication_requirement::AuthenticationRequirement::Rule(
        frontend_contract::contract_str::ContractStr::from(admin_rule.as_str().get()),
    )
}
