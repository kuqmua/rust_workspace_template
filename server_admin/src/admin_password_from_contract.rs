pub(super) fn admin_password_from_contract(
    value: server_admin_contract::domain_types::AdminPassword,
) -> Result<super::super::AdminPassword, super::super::AdminPasswordTryFromStringError> {
    super::super::AdminPassword::try_from(value.into_inner())
}
