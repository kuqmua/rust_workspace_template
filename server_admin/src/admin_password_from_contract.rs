pub(crate) fn admin_password_from_contract(
    value: server_admin_contract::admin_password::AdminPassword,
) -> Result<
    crate::admin_password::AdminPassword,
    crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError,
> {
    crate::admin_password::AdminPassword::try_from(value.into_inner())
}
