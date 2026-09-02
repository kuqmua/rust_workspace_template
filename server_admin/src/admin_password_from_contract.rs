pub(crate) fn admin_password_from_contract(
    admin_password: server_admin_contract::admin_password::AdminPassword,
) -> Result<
    crate::runtime_admin_password::RuntimeAdminPassword,
    crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError,
> {
    crate::runtime_admin_password::RuntimeAdminPassword::try_from(admin_password.into_inner())
}
