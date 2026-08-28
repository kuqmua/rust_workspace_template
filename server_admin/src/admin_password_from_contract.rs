pub(crate) fn admin_password_from_contract(
    value: server_admin_contract::domain_types::AdminPassword,
) -> Result<crate::AdminPassword, crate::AdminPasswordTryFromStringError> {
    crate::AdminPassword::try_from(value.into_inner())
}
