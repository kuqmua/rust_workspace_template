pub(crate) fn admin_new_password_from_contract(
    admin_new_password: server_admin_contract::admin_new_password::AdminNewPassword,
) -> Result<
    crate::runtime_admin_password::RuntimeAdminPassword,
    crate::admin_password_try_from_string_error::AdminPasswordTryFromStringError,
> {
    crate::runtime_admin_password::RuntimeAdminPassword::try_from(admin_new_password.into_inner())
}
